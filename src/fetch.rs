use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Error, Result, bail};
use futures_util::StreamExt;
use reqwest::{Client, StatusCode};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;
use tokio::net::lookup_host;
use tokio::sync::Mutex;
use url::Url;

use crate::config::validate_https_url;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(90);
const MAX_ATTEMPTS: u32 = 5;
const MAX_REDIRECTS: usize = 5;
const MAX_SOURCE_BYTES: u64 = 512 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const MAX_LINE_BYTES: usize = 4096;
const MAX_LINES: u64 = 10_000_000;
const HEX: &[u8; 16] = b"0123456789abcdef";

#[derive(Clone, Default)]
pub struct DownloadBudget(Arc<Mutex<u64>>);

impl DownloadBudget {
    async fn consume(&self, amount: u64) -> Result<()> {
        let mut used = self.0.lock().await;
        *used += amount;
        if *used > MAX_TOTAL_BYTES {
            bail!("aggregate download limit of {MAX_TOTAL_BYTES} bytes exceeded");
        }
        Ok(())
    }
}

pub struct FetchedSource {
    pub hash: String,
    pub path: PathBuf,
}

pub fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_public_v4(ip),
        IpAddr::V6(ip) => is_public_v6(ip),
    }
}

fn is_public_v4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    !(ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
        || octets[0] == 0
        || octets[0] >= 224
        || (octets[0] == 100 && (64..=127).contains(&octets[1]))
        || (octets[0] == 192 && octets[1] == 0 && octets[2] == 0)
        || (octets[0] == 198 && (octets[1] == 18 || octets[1] == 19)))
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return is_public_v4(mapped);
    }
    let segments = ip.segments();
    !(ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_unique_local()
        || ip.is_unicast_link_local()
        || ip.is_multicast()
        || segments[0] == 0
        || (segments[0] == 0x0064 && segments[1] == 0xff9b)
        || (segments[0] == 0x0100 && segments[1..4] == [0, 0, 0])
        || (segments[0] & 0xffc0) == 0xfec0
        || (segments[0] == 0x2001 && segments[1] == 0x0002)
        || (segments[0] == 0x2001 && segments[1] == 0x0db8)
        || (segments[0] == 0x2001 && (0x0020..=0x002f).contains(&segments[1])))
}

async fn pinned_client(url: &Url) -> Result<Client> {
    let host = url.host_str().context("URL has no hostname")?;
    let mut addresses: Vec<SocketAddr> = lookup_host((host, 443))
        .await
        .with_context(|| format!("DNS lookup failed for {host}"))?
        .collect();
    if addresses.is_empty() {
        bail!("{host} did not resolve");
    }
    if let Some(address) = addresses.iter().find(|address| !is_public_ip(address.ip())) {
        bail!("{host} resolved to non-public address {}", address.ip());
    }
    addresses.sort_unstable_by_key(SocketAddr::is_ipv6);
    Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(READ_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .user_agent(concat!(
            "host-judge/",
            env!("CARGO_PKG_VERSION"),
            " (+https://github.com/scottdraper8/host-judge)"
        ))
        .resolve_to_addrs(host, &addresses)
        .build()
        .context("failed to construct HTTP client")
}

pub async fn fetch(
    url: &str,
    destination: &Path,
    budget: &DownloadBudget,
) -> Result<FetchedSource> {
    let mut last_error = None;
    for attempt in 1..=MAX_ATTEMPTS {
        match fetch_once(url, destination, budget).await {
            Ok(result) => return Ok(result),
            Err(error) => {
                let _ = tokio::fs::remove_file(destination).await;
                if !is_retryable(&error) {
                    return Err(error).with_context(|| format!("refused to fetch {url}"));
                }
                last_error = Some(error);
                if attempt < MAX_ATTEMPTS {
                    tokio::time::sleep(Duration::from_secs(u64::from(attempt) * 2)).await;
                }
            }
        }
    }
    Err(last_error.context("fetch failed without an error")?)
        .with_context(|| format!("failed to fetch {url} after {MAX_ATTEMPTS} attempts"))
}

fn is_retryable(error: &Error) -> bool {
    let message = error.to_string();
    message.starts_with("DNS lookup failed")
        || message.ends_with("did not resolve")
        || message.starts_with("retryable HTTP status")
        || error.chain().any(|cause| {
            cause
                .downcast_ref::<reqwest::Error>()
                .is_some_and(|request| {
                    request.is_connect() || request.is_timeout() || request.is_body()
                })
        })
}

async fn fetch_once(
    url: &str,
    destination: &Path,
    budget: &DownloadBudget,
) -> Result<FetchedSource> {
    let mut current = validate_https_url(url, "source URL")?;
    for redirect in 0..=MAX_REDIRECTS {
        let client = pinned_client(&current).await?;
        let response = client
            .get(current.clone())
            .header(reqwest::header::ACCEPT_ENCODING, "identity")
            .send()
            .await?;
        if response.status().is_redirection() {
            if redirect == MAX_REDIRECTS {
                bail!("too many redirects while fetching {url}");
            }
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .context("redirect omitted Location header")?
                .to_str()?;
            current = validate_https_url(current.join(location)?.as_str(), "redirect URL")?;
            continue;
        }
        if matches!(
            response.status(),
            StatusCode::REQUEST_TIMEOUT | StatusCode::TOO_MANY_REQUESTS
        ) || response.status().is_server_error()
        {
            bail!("retryable HTTP status {} from {current}", response.status());
        }
        let response = response.error_for_status()?;
        if let Some(encoding) = response.headers().get(reqwest::header::CONTENT_ENCODING) {
            let encoding = encoding.to_str()?;
            if !encoding.is_empty() && !encoding.eq_ignore_ascii_case("identity") {
                bail!("unsupported Content-Encoding {encoding:?} from {current}");
            }
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_SOURCE_BYTES)
        {
            bail!("source declares more than {MAX_SOURCE_BYTES} bytes");
        }
        let mut output = tokio::fs::File::create(destination).await?;
        let mut stream = response.bytes_stream();
        let mut digest = Sha256::new();
        let mut bytes = 0_u64;
        let mut lines = 0_u64;
        let mut line_bytes = 0_usize;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            budget.consume(chunk.len() as u64).await?;
            bytes += chunk.len() as u64;
            if bytes > MAX_SOURCE_BYTES {
                bail!("source exceeds {MAX_SOURCE_BYTES} bytes");
            }
            for byte in &chunk {
                if *byte == b'\n' {
                    lines += 1;
                    line_bytes = 0;
                } else {
                    line_bytes += 1;
                    if line_bytes > MAX_LINE_BYTES {
                        bail!("source contains a line longer than {MAX_LINE_BYTES} bytes");
                    }
                }
            }
            if lines > MAX_LINES {
                bail!("source exceeds {MAX_LINES} lines");
            }
            digest.update(&chunk);
            output.write_all(&chunk).await?;
        }
        if line_bytes > 0 {
            lines += 1;
        }
        if lines > MAX_LINES {
            bail!("source exceeds {MAX_LINES} lines");
        }
        output.sync_all().await?;
        let digest = digest.finalize();
        let mut hash = String::with_capacity(digest.len() * 2);
        for byte in digest {
            hash.push(char::from(HEX[usize::from(byte >> 4)]));
            hash.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        return Ok(FetchedSource {
            hash,
            path: destination.to_owned(),
        });
    }
    bail!("too many redirects while fetching {url}")
}

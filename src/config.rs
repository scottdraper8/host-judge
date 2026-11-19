use std::collections::HashSet;
use std::fs;
use std::net::IpAddr;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use url::Url;

const MAX_SOURCE_NAME_LENGTH: usize = 200;
const MAX_METADATA_LENGTH: usize = 500;
const MAX_URL_LENGTH: usize = 2_048;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub nsfw: bool,
    pub maintainer_name: Option<String>,
    pub maintainer_url: Option<String>,
    pub maintainer_description: Option<String>,
}

#[derive(Default)]
pub struct Whitelist {
    exact: HashSet<String>,
    wildcard_suffixes: Vec<String>,
}

impl Whitelist {
    pub fn matches(&self, domain: &str) -> bool {
        self.exact.contains(domain)
            || self.wildcard_suffixes.iter().any(|suffix| {
                domain == suffix
                    || domain
                        .strip_suffix(suffix)
                        .is_some_and(|prefix| prefix.ends_with('.'))
            })
    }
}

pub fn validate_https_url(raw: &str, field: &str) -> Result<Url> {
    if raw.len() > MAX_URL_LENGTH || raw.contains('\\') || raw.chars().any(char::is_control) {
        bail!(
            "{field} must be at most {MAX_URL_LENGTH} bytes and contain no backslashes or control characters"
        );
    }
    let parsed = Url::parse(raw).with_context(|| format!("{field} is not a valid URL"))?;
    if parsed.scheme() != "https" {
        bail!("{field} must use HTTPS");
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        bail!("{field} must not contain credentials");
    }
    if parsed.fragment().is_some() {
        bail!("{field} must not contain a fragment");
    }
    if parsed.port().is_some_and(|port| port != 443) {
        bail!("{field} must use the default HTTPS port");
    }
    let host = parsed
        .host_str()
        .with_context(|| format!("{field} must contain a hostname"))?;
    if let Ok(ip) = host.parse::<IpAddr>() {
        if !super::fetch::is_public_ip(ip) {
            bail!("{field} resolves to a non-public IP address");
        }
    } else if host.len() > 253
        || !host.is_ascii()
        || host.ends_with('.')
        || host.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
    {
        bail!("{field} contains an invalid hostname");
    }
    Ok(parsed)
}

pub fn load_sources(path: &Path) -> Result<Vec<SourceConfig>> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let sources: Vec<SourceConfig> =
        serde_json::from_slice(&bytes).with_context(|| format!("invalid {}", path.display()))?;
    if sources.is_empty() || sources.len() > 128 {
        bail!("source configuration must contain between 1 and 128 entries");
    }
    let mut names = HashSet::new();
    let mut urls = HashSet::new();
    for (index, source) in sources.iter().enumerate() {
        let entry = index + 1;
        validate_text(
            &source.name,
            MAX_SOURCE_NAME_LENGTH,
            &format!("entry {entry} name"),
        )?;
        validate_https_url(&source.url, &format!("entry {entry} URL"))?;
        if !names.insert(source.name.to_lowercase()) {
            bail!("entry {entry} has a duplicate source name");
        }
        if !urls.insert(source.url.clone()) {
            bail!("entry {entry} has a duplicate source URL");
        }
        for (label, value) in [
            ("maintainer_name", source.maintainer_name.as_deref()),
            (
                "maintainer_description",
                source.maintainer_description.as_deref(),
            ),
        ] {
            if let Some(text) = value {
                validate_text(text, MAX_METADATA_LENGTH, &format!("entry {entry} {label}"))?;
            }
        }
        if let Some(url) = source.maintainer_url.as_deref() {
            validate_text(
                url,
                MAX_METADATA_LENGTH,
                &format!("entry {entry} maintainer_url"),
            )?;
            validate_https_url(url, &format!("entry {entry} maintainer_url"))?;
        }
    }
    Ok(sources)
}

fn validate_text(value: &str, limit: usize, field: &str) -> Result<()> {
    if value.is_empty() || value.trim() != value || value.len() > limit {
        bail!("{field} must be non-empty, trimmed, and at most {limit} bytes");
    }
    if value.chars().any(char::is_control) {
        bail!("{field} must not contain control characters");
    }
    Ok(())
}

pub fn load_whitelist(path: &Path) -> Result<Whitelist> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Whitelist::default());
        }
        Err(error) => {
            return Err(error).with_context(|| format!("failed to read {}", path.display()));
        }
    };
    let mut whitelist = Whitelist::default();
    for raw in content.lines() {
        let line = raw.trim().to_ascii_lowercase();
        if line.is_empty() || line.starts_with(['!', '#']) {
            continue;
        }
        let value =
            if let Some(domain) = line.strip_prefix("@@||").and_then(|v| v.strip_suffix('^')) {
                format!("*.{domain}")
            } else {
                line
            };
        if let Some(suffix) = value.strip_prefix("*.") {
            if super::domain::is_valid_domain(suffix) {
                whitelist.wildcard_suffixes.push(suffix.to_owned());
            }
        } else if super::domain::is_valid_domain(&value) {
            whitelist.exact.insert(value);
        }
    }
    Ok(whitelist)
}

pub fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 || !domain.contains('.') || !domain.is_ascii() {
        return false;
    }
    domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

pub fn extract_domain(raw: &str) -> Option<String> {
    let line = raw.trim();
    if line.is_empty() || line.starts_with(['#', '!', '[']) {
        return None;
    }
    let candidate = if let Some(rest) = line.strip_prefix("||") {
        rest.split('^').next()?
    } else {
        let mut fields = line.split_ascii_whitespace();
        let first = fields.next()?;
        if matches!(first, "0.0.0.0" | "127.0.0.1" | "::1") {
            fields.next()?
        } else if fields.next().is_none() {
            first
        } else {
            return None;
        }
    };
    let domain = candidate.to_ascii_lowercase();
    if matches!(
        domain.as_str(),
        "localhost" | "ip6-localhost" | "ip6-loopback"
    ) || !is_valid_domain(&domain)
    {
        return None;
    }
    Some(domain)
}

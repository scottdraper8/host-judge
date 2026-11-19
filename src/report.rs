use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};
use jiff::Timestamp;

use crate::config::SourceConfig;
use crate::pipeline::Analysis;

const START: &str = "<!-- STATS_START -->";
const END: &str = "<!-- STATS_END -->";

pub fn candidate_reasons(unique: u64, stale_days: Option<i64>) -> Vec<String> {
    let mut reasons = Vec::new();
    if unique <= 1_000 {
        reasons.push(format!(
            "{} unique domain{}",
            format_number(unique),
            if unique == 1 { "" } else { "s" }
        ));
    }
    if let Some(days) = stale_days {
        reasons.push(format!("unchanged {days} days"));
    }
    reasons
}

pub fn update_atomic(
    path: &Path,
    sources: &[SourceConfig],
    analysis: &Analysis,
    stale: &[Option<i64>],
    timestamp: Timestamp,
) -> Result<()> {
    let original =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let start = original
        .find(START)
        .context("README is missing STATS_START")?;
    let end = original.find(END).context("README is missing STATS_END")?;
    if end <= start {
        bail!("README stats markers are out of order");
    }
    let general: Vec<usize> = sources
        .iter()
        .enumerate()
        .filter_map(|(index, source)| (!source.nsfw).then_some(index))
        .collect();
    let nsfw: Vec<usize> = sources
        .iter()
        .enumerate()
        .filter_map(|(index, source)| source.nsfw.then_some(index))
        .collect();
    let section = format!(
        "{START}\n\n## Latest run\n\nLast updated: {}\n\n- General domains: {}\n- Total domains including NSFW: {}\n- Whitelisted domains excluded: {}\n\n### General host lists\n\n{}\n\n### NSFW host lists\n\n{}\n\n> [!NOTE]\n> **Unique Contribution** is the number of domains that would disappear if a\n> source were removed. A source is a **Removal Candidate** when it contributes\n> 1,000 or fewer unique domains or has no observed content change for 30 days.\n> Sources are never removed automatically.\n\n{END}",
        timestamp,
        format_number(analysis.general_count),
        format_number(analysis.all_count),
        format_number(analysis.whitelisted_count),
        table(sources, analysis, stale, &general, false),
        table(sources, analysis, stale, &nsfw, true),
    );
    let result = format!(
        "{}{}{}",
        &original[..start],
        section,
        &original[end + END.len()..]
    );
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    temporary.write_all(result.as_bytes())?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

fn table(
    sources: &[SourceConfig],
    analysis: &Analysis,
    stale: &[Option<i64>],
    indices: &[usize],
    nsfw: bool,
) -> String {
    let contributions = if nsfw {
        &analysis.contrib_all
    } else {
        &analysis.contrib_general
    };
    let mut sorted = indices.to_vec();
    sorted.sort_unstable_by_key(|index| std::cmp::Reverse(contributions[*index]));
    let mut rows = String::new();
    for index in sorted {
        let source = &sources[index];
        let reasons = candidate_reasons(contributions[index], stale[index]);
        let candidate = if reasons.is_empty() {
            "No".to_owned()
        } else {
            format!(
                "<strong>Yes</strong> — {}",
                escape_html(&reasons.join("; "))
            )
        };
        let _ = writeln!(
            rows,
            "<tr><td><a href=\"{}\">{}</a></td><td>{}</td><td>{}</td><td>{candidate}</td></tr>",
            escape_html(&source.url),
            escape_html(&source.name),
            format_number(analysis.source_counts[index]),
            format_number(contributions[index]),
        );
    }
    format!(
        "<table>\n<!-- markdownlint-disable MD013 -->\n<thead><tr><th>Source List</th><th>Total Domains</th><th>Unique Contribution</th><th>Removal Candidate</th></tr></thead>\n<tbody>\n{rows}</tbody>\n<!-- markdownlint-enable MD013 -->\n</table>"
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn format_number(value: u64) -> String {
    let digits = value.to_string();
    let mut result = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, byte) in digits.bytes().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            result.push(',');
        }
        result.push(char::from(byte));
    }
    result
}

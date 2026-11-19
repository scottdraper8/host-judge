use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::SourceConfig;

pub const STALE_THRESHOLD_DAYS: i64 = 30;
pub const ANALYSIS_INTERVAL_DAYS: i64 = 7;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SourceState {
    pub url: String,
    pub content_hash: String,
    pub last_fetch_date: String,
    pub last_changed_date: String,
    pub fetch_count: u64,
    pub change_count: u64,
    #[serde(default)]
    pub metadata: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnalysisState {
    #[serde(default)]
    pub sources: BTreeMap<String, SourceState>,
    #[serde(default)]
    pub last_analysis: String,
    #[serde(default)]
    pub analysis_count: u64,
    #[serde(default)]
    pub skipped_analyses: u64,
}

pub fn load(path: &Path) -> Result<AnalysisState> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .with_context(|| format!("invalid state file {}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(AnalysisState::default()),
        Err(error) => Err(error).with_context(|| format!("failed to read {}", path.display())),
    }
}

pub fn save_atomic(path: &Path, state: &AnalysisState) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer_pretty(&mut temporary, state)?;
    temporary.write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

pub fn stale_days(source: &SourceState, now: Timestamp) -> Option<i64> {
    let changed = source.last_changed_date.parse::<Timestamp>().ok()?;
    let days = now.as_second().saturating_sub(changed.as_second()) / 86_400;
    (days >= STALE_THRESHOLD_DAYS).then_some(days)
}

pub fn should_force(state: &AnalysisState, now: Timestamp) -> bool {
    if state.last_analysis.is_empty() {
        return true;
    }
    state
        .last_analysis
        .parse::<Timestamp>()
        .map_or(true, |last| {
            now.as_second().saturating_sub(last.as_second()) / 86_400 >= ANALYSIS_INTERVAL_DAYS
        })
}

pub fn update_source(
    state: &mut AnalysisState,
    source: &SourceConfig,
    hash: &str,
    now: Timestamp,
) -> bool {
    let timestamp = now.to_string();
    if let Some(existing) = state.sources.get_mut(&source.name) {
        let changed = existing.content_hash != hash || existing.url != source.url;
        existing.url.clone_from(&source.url);
        existing.last_fetch_date.clone_from(&timestamp);
        existing.fetch_count += 1;
        if changed {
            hash.clone_into(&mut existing.content_hash);
            existing.last_changed_date = timestamp;
            existing.change_count += 1;
            existing.metadata.remove("stale_candidate_reported");
        }
        changed
    } else {
        state.sources.insert(
            source.name.clone(),
            SourceState {
                url: source.url.clone(),
                content_hash: hash.to_owned(),
                last_fetch_date: timestamp.clone(),
                last_changed_date: timestamp,
                fetch_count: 1,
                change_count: 1,
                metadata: BTreeMap::new(),
            },
        );
        true
    }
}

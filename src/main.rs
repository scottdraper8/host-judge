mod config;
mod domain;
mod fetch;
mod pipeline;
mod report;
mod state;

use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use futures_util::{StreamExt, stream};
use jiff::Timestamp;
use serde_json::{Value, json};

use crate::config::{SourceConfig, load_sources, load_whitelist};
use crate::fetch::{DownloadBudget, FetchedSource};
use crate::pipeline::Analysis;
use crate::state::AnalysisState;

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    #[arg(long)]
    force: bool,
}

#[tokio::main]
async fn main() {
    if let Err(error) = run(Arguments::parse()).await {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}

async fn run(arguments: Arguments) -> Result<()> {
    let now = Timestamp::now();
    let sources = load_sources(Path::new("blocklists.json"))?;
    let whitelist = load_whitelist(Path::new("whitelist.txt"))?;
    let mut analysis_state = state::load(Path::new("state.json"))?;
    analysis_state
        .sources
        .retain(|name, _| sources.iter().any(|source| source.name == *name));
    let temporary = tempfile::Builder::new().prefix("host-judge-").tempdir()?;

    println!(
        "Fetching {} host lists with bounded concurrency...",
        sources.len()
    );
    let fetched = fetch_all(&sources, temporary.path()).await?;
    let mut changed = false;
    for (source, body) in sources.iter().zip(&fetched) {
        changed |= state::update_source(&mut analysis_state, source, &body.hash, now);
    }
    let stale_sources: Vec<Option<i64>> = sources
        .iter()
        .map(|source| {
            analysis_state
                .sources
                .get(&source.name)
                .and_then(|entry| state::stale_days(entry, now))
        })
        .collect();
    let newly_stale = sources.iter().enumerate().any(|(index, source)| {
        stale_sources[index].is_some()
            && analysis_state
                .sources
                .get(&source.name)
                .is_some_and(|entry| {
                    entry.metadata.get("stale_candidate_reported") != Some(&Value::Bool(true))
                })
    });
    if !arguments.force && !changed && !newly_stale && !state::should_force(&analysis_state, now) {
        println!("No content changes; preserving the current analysis.");
        analysis_state.skipped_analyses += 1;
        state::save_atomic(Path::new("state.json"), &analysis_state)?;
        return Ok(());
    }

    println!("Sorting and analyzing domains...");
    let analysis = pipeline::analyze(&sources, &fetched, &whitelist, temporary.path())?;
    update_metadata(&sources, &analysis, &stale_sources, &mut analysis_state)?;
    analysis_state.last_analysis = now.to_string();
    analysis_state.analysis_count += 1;
    report::update_atomic(
        Path::new("README.md"),
        &sources,
        &analysis,
        &stale_sources,
        now,
    )?;
    state::save_atomic(Path::new("state.json"), &analysis_state)?;
    println!(
        "Analysis complete: {} general domains, {} total domains; {} whitelisted domains excluded.",
        report::format_number(analysis.general_count),
        report::format_number(analysis.all_count),
        report::format_number(analysis.whitelisted_count)
    );
    Ok(())
}

async fn fetch_all(sources: &[SourceConfig], directory: &Path) -> Result<Vec<FetchedSource>> {
    let budget = DownloadBudget::default();
    let results: Vec<Result<(usize, FetchedSource)>> = stream::iter(sources.iter().enumerate())
        .map(|(index, source)| {
            let budget = budget.clone();
            let path = directory.join(format!("source-{index}.list"));
            async move {
                fetch::fetch(&source.url, &path, &budget)
                    .await
                    .with_context(|| format!("{} ({})", source.name, source.url))
                    .map(|fetched| (index, fetched))
            }
        })
        .buffer_unordered(5)
        .collect()
        .await;
    let mut fetched: Vec<Option<FetchedSource>> = (0..sources.len()).map(|_| None).collect();
    for result in results {
        let (index, source) = result?;
        fetched[index] = Some(source);
    }
    fetched
        .into_iter()
        .enumerate()
        .map(|(index, source)| source.with_context(|| format!("source {index} was not fetched")))
        .collect()
}

fn update_metadata(
    sources: &[SourceConfig],
    analysis: &Analysis,
    stale_sources: &[Option<i64>],
    analysis_state: &mut AnalysisState,
) -> Result<()> {
    for (index, source) in sources.iter().enumerate() {
        let unique = if source.nsfw {
            analysis.contrib_all[index]
        } else {
            analysis.contrib_general[index]
        };
        let reasons = report::candidate_reasons(unique, stale_sources[index]);
        let entry = analysis_state
            .sources
            .get_mut(&source.name)
            .with_context(|| format!("missing state for {}", source.name))?;
        entry.metadata.insert(
            "domain_count".to_owned(),
            json!(analysis.source_counts[index]),
        );
        entry
            .metadata
            .insert("unique_contribution".to_owned(), json!(unique));
        entry
            .metadata
            .insert("removal_candidate".to_owned(), json!(!reasons.is_empty()));
        entry
            .metadata
            .insert("candidate_reasons".to_owned(), json!(reasons));
        if stale_sources[index].is_some() {
            entry
                .metadata
                .insert("stale_candidate_reported".to_owned(), Value::Bool(true));
        } else {
            entry.metadata.remove("stale_candidate_reported");
        }
    }
    Ok(())
}

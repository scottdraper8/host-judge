use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::{SourceConfig, Whitelist};
use crate::domain::extract_domain;
use crate::fetch::FetchedSource;

const RUN_TARGET_BYTES: usize = 96 * 1024 * 1024;
const MAX_DOMAINS_PER_SOURCE: u64 = 10_000_000;
const MAX_TOTAL_DOMAINS: u64 = 30_000_000;

#[derive(Eq, PartialEq)]
struct Record {
    domain: String,
    source: u16,
    general: bool,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.domain
            .cmp(&other.domain)
            .then(self.source.cmp(&other.source))
            .then(self.general.cmp(&other.general))
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Analysis {
    pub all_count: u64,
    pub general_count: u64,
    pub whitelisted_count: u64,
    pub source_counts: Vec<u64>,
    pub contrib_all: Vec<u64>,
    pub contrib_general: Vec<u64>,
}

pub fn analyze(
    sources: &[SourceConfig],
    fetched: &[FetchedSource],
    whitelist: &Whitelist,
    temporary: &Path,
) -> Result<Analysis> {
    if sources.len() > 128 || sources.len() != fetched.len() {
        bail!("analysis supports at most 128 sources and requires one fetch per source");
    }
    let mut records = Vec::new();
    let mut record_bytes = 0_usize;
    let mut runs = Vec::new();
    let mut parsed_counts = vec![0_u64; sources.len()];
    let mut total = 0_u64;

    for (source_id, fetched_source) in fetched.iter().enumerate() {
        let input = File::open(&fetched_source.path)?;
        for line in BufReader::new(input).lines() {
            let line = line.context("source is not valid UTF-8")?;
            if let Some(domain) = extract_domain(&line) {
                parsed_counts[source_id] += 1;
                total += 1;
                if parsed_counts[source_id] > MAX_DOMAINS_PER_SOURCE {
                    bail!(
                        "{} exceeds {MAX_DOMAINS_PER_SOURCE} parsed domains",
                        sources[source_id].name
                    );
                }
                if total > MAX_TOTAL_DOMAINS {
                    bail!("analysis exceeds {MAX_TOTAL_DOMAINS} parsed domains");
                }
                record_bytes += domain.len() + 16;
                records.push(Record {
                    domain,
                    source: u16::try_from(source_id)?,
                    general: !sources[source_id].nsfw,
                });
                if record_bytes >= RUN_TARGET_BYTES {
                    runs.push(write_run(temporary, runs.len(), &mut records)?);
                    record_bytes = 0;
                }
            }
        }
    }
    if !records.is_empty() {
        runs.push(write_run(temporary, runs.len(), &mut records)?);
    }
    merge_runs(&runs, sources.len(), whitelist)
}

fn write_run(directory: &Path, index: usize, records: &mut Vec<Record>) -> Result<PathBuf> {
    records.sort_unstable();
    records.dedup_by(|left, right| left.domain == right.domain && left.source == right.source);
    let path = directory.join(format!("run-{index:04}.bin"));
    let mut output = BufWriter::new(File::create(&path)?);
    for record in records.drain(..) {
        let length = u16::try_from(record.domain.len())?;
        output.write_all(&length.to_le_bytes())?;
        output.write_all(record.domain.as_bytes())?;
        output.write_all(&record.source.to_le_bytes())?;
        output.write_all(&[u8::from(record.general)])?;
    }
    output.flush()?;
    Ok(path)
}

#[derive(Eq, PartialEq)]
struct HeapRecord {
    record: Record,
    run: usize,
}

impl Ord for HeapRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.record
            .cmp(&other.record)
            .then(self.run.cmp(&other.run))
    }
}

impl PartialOrd for HeapRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn merge_runs(paths: &[PathBuf], source_len: usize, whitelist: &Whitelist) -> Result<Analysis> {
    let mut readers: Vec<RunReader> = paths
        .iter()
        .map(|path| File::open(path).map(RunReader::new))
        .collect::<std::io::Result<_>>()?;
    let mut heap = BinaryHeap::new();
    for (run, reader) in readers.iter_mut().enumerate() {
        if let Some(record) = reader.next_record()? {
            heap.push(Reverse(HeapRecord { record, run }));
        }
    }
    let mut analysis = Analysis {
        all_count: 0,
        general_count: 0,
        whitelisted_count: 0,
        source_counts: vec![0; source_len],
        contrib_all: vec![0; source_len],
        contrib_general: vec![0; source_len],
    };
    let mut current = String::new();
    let mut mask_all = 0_u128;
    let mut mask_general = 0_u128;
    while let Some(Reverse(item)) = heap.pop() {
        if !current.is_empty() && current != item.record.domain {
            flush_domain(&current, mask_all, mask_general, whitelist, &mut analysis);
            mask_all = 0;
            mask_general = 0;
        }
        current.clone_from(&item.record.domain);
        let bit = 1_u128 << item.record.source;
        mask_all |= bit;
        if item.record.general {
            mask_general |= bit;
        }
        if let Some(record) = readers[item.run].next_record()? {
            heap.push(Reverse(HeapRecord {
                record,
                run: item.run,
            }));
        }
    }
    if !current.is_empty() {
        flush_domain(&current, mask_all, mask_general, whitelist, &mut analysis);
    }
    Ok(analysis)
}

fn flush_domain(
    domain: &str,
    all: u128,
    general: u128,
    whitelist: &Whitelist,
    analysis: &mut Analysis,
) {
    if whitelist.matches(domain) {
        analysis.whitelisted_count += 1;
        return;
    }
    let mut source_bits = all;
    while source_bits != 0 {
        analysis.source_counts[source_bits.trailing_zeros() as usize] += 1;
        source_bits &= source_bits - 1;
    }
    analysis.all_count += 1;
    if general != 0 {
        analysis.general_count += 1;
    }
    if all.is_power_of_two() {
        analysis.contrib_all[all.trailing_zeros() as usize] += 1;
    }
    if general.is_power_of_two() {
        analysis.contrib_general[general.trailing_zeros() as usize] += 1;
    }
}

struct RunReader(BufReader<File>);

impl RunReader {
    fn new(file: File) -> Self {
        Self(BufReader::new(file))
    }

    fn next_record(&mut self) -> Result<Option<Record>> {
        let mut length = [0_u8; 2];
        let read = self.0.read(&mut length)?;
        if read == 0 {
            return Ok(None);
        }
        if read < length.len() {
            self.0.read_exact(&mut length[read..])?;
        }
        let length = usize::from(u16::from_le_bytes(length));
        let mut domain = vec![0_u8; length];
        self.0.read_exact(&mut domain)?;
        let mut source = [0_u8; 2];
        self.0.read_exact(&mut source)?;
        let mut general = [0_u8; 1];
        self.0.read_exact(&mut general)?;
        Ok(Some(Record {
            domain: String::from_utf8(domain)?,
            source: u16::from_le_bytes(source),
            general: general[0] == 1,
        }))
    }
}

<div align="center">

# Host Judge

[![Analyze Host Lists](https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fscottdraper8%2Fhost-judge%2Fbadges%2Fanalyze-host-lists.json&logo=github&logoColor=white&labelColor=6272a4)](https://github.com/scottdraper8/host-judge/actions/workflows/analyze-host-lists.yml)
[![Python 3.11+](https://img.shields.io/badge/Python-3.11+-bd93f9?logo=python&logoColor=white&labelColor=6272a4)](https://www.python.org/downloads/)
[![uv](https://img.shields.io/badge/uv-managed-ff79c6?logo=astral&logoColor=white&labelColor=6272a4)](https://docs.astral.sh/uv/)
[![pre-commit](https://img.shields.io/badge/pre--commit-4.6-f1fa8c?logo=pre-commit&logoColor=282a36&labelColor=6272a4)](https://github.com/pre-commit/pre-commit)
[![curl-cffi](https://img.shields.io/badge/curl--cffi-0.15-8be9fd?logo=curl&logoColor=white&labelColor=6272a4)](https://github.com/yifeikong/curl_cffi)

---

Host-list relevance monitor and whitelist workspace. Tracks configured
sources for staleness, unique contribution, and removal-candidate analysis.

---

</div>

## Purpose

Host Judge polls the configured host lists weekly and publishes analytics to this
README. It measures how many domains each source uniquely contributes and flags
sources as removal candidates when they contribute 50 or fewer unique domains or
show no observed content change for 30 days.

The repository-level [`whitelist.txt`](whitelist.txt) is applied before domain
counts and contribution statistics are calculated, so the report reflects the
effective policy used for the maintained devices.

## How It Works

Runs automatically once a week via GitHub Actions. Sources are fetched in
parallel, normalized, filtered through the whitelist, and deduplicated for
provenance and contribution analysis. The workflow updates only the README and
analysis state; it does not generate or publish a combined host list.

<!-- STATS_START -->

## Latest Run

<div align="center">

![General Domains](https://img.shields.io/badge/General_Domains-5,844,910-8be9fd?style=for-the-badge&labelColor=6272a4)
![Total Domains](https://img.shields.io/badge/Total_Domains_(with_NSFW)-10,744,100-ff79c6?style=for-the-badge&labelColor=6272a4)
![Last Updated](https://img.shields.io/badge/Last_Updated-2026--09--13_08:11:07_UTC-50fa7b?style=for-the-badge&labelColor=6272a4)

### General Host Lists

<table align="center">
<!-- markdownlint-disable MD013 -->
<thead>
<tr>
<th>Source List</th>
<th>Total Domains</th>
<th>Unique Contribution</th>
<th>Removal Candidate</th>
</tr>
</thead>
<tbody>
<tr><td><a href="https://raw.githubusercontent.com/hagezi/nrd/main/adblock/dga30.txt">HaGeZi DGA 30 Days</a></td><td>2,410,601</td><td>2,342,926</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/tif.txt">HaGeZi Threat Intelligence</a></td><td>2,432,074</td><td>2,172,438</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Malware.txt">RPiList Malware</a></td><td>593,666</td><td>435,804</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/AdguardDNS.txt">AdGuard DNS Filter</a></td><td>177,306</td><td>74,686</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/pro.txt">HaGeZi Multi-pro Extended</a></td><td>222,511</td><td>71,789</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/RooneyMcNibNug/pihole-stuff/master/SNAFU.txt">SNAFU</a></td><td>74,798</td><td>71,633</td><td>No</td></tr>
<tr><td><a href="https://lists.cyberhost.uk/malware.txt">Cyber Threat Coalition Malware</a></td><td>85,191</td><td>69,062</td><td>No</td></tr>
<tr><td><a href="https://big.oisd.nl">OISD Big List</a></td><td>245,444</td><td>22,911</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts">Steven Black&#x27;s Unified Hosts</a></td><td>79,962</td><td>22,643</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Phishing.txt">RPiList Phishing</a></td><td>157,281</td><td>20,449</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Easyprivacy.txt">EasyPrivacy</a></td><td>43,014</td><td>12,261</td><td>No</td></tr>
<tr><td><a href="https://hostfiles.frogeye.fr/firstparty-trackers-hosts.txt">First-Party Trackers</a></td><td>14,754</td><td>11,516</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Crypto.txt">Prigent Crypto</a></td><td>11,491</td><td>9,790</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/bigdargon/hostsVN/master/hosts">hostsVN</a></td><td>18,401</td><td>5,959</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt">KADhosts</a></td><td>43,604</td><td>3,942</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/DandelionSprout/adfilt/master/Alternate%20versions%20Anti-Malware%20List/AntiMalwareHosts.txt">DandelionSprout Anti-Malware</a></td><td>12,362</td><td>3,028</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Ads.txt">Prigent Ads</a></td><td>4,270</td><td>1,523</td><td>No</td></tr>
<tr><td><a href="https://malware-filter.gitlab.io/malware-filter/phishing-filter-hosts.txt">Phishing Hosts</a></td><td>39,088</td><td>1,277</td><td>No</td></tr>
</tbody>
</table>
<!-- markdownlint-enable MD013 -->

### NSFW Host Lists

<table align="center">
<!-- markdownlint-disable MD013 -->
<thead>
<tr>
<th>Source List</th>
<th>Total Domains</th>
<th>Unique Contribution</th>
<th>Removal Candidate</th>
</tr>
</thead>
<tbody>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Adult.txt">Prigent Adult</a></td><td>4,590,338</td><td>4,431,798</td><td>No</td></tr>
<tr><td><a href="https://nsfw.oisd.nl">OISD NSFW</a></td><td>464,259</td><td>275,932</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/nsfw.txt">HaGeZi NSFW</a></td><td>74,255</td><td>4,006</td><td>No</td></tr>
</tbody>
</table>
<!-- markdownlint-enable MD013 -->

</div>

> [!NOTE]
> **Unique Contribution** is the number of domains that would disappear if a
> source were removed. A source is a **Removal Candidate** when it contributes
> 50 or fewer unique domains or has no observed content change for 30 days.
> Sources are never removed automatically.

<!-- STATS_END -->

---

## Local Development Setup

**Prerequisites:**

- Python 3.11 or higher
- [uv](https://docs.astral.sh/uv/getting-started/installation/) for dependency
  management and the project environment

**Clone and setup:**

```bash
git clone https://github.com/scottdraper8/host-judge.git
cd host-judge
uv sync
```

**Install pre-commit hooks:**

```bash
uv run pre-commit install
```

**Run locally:**

```bash
# Analyze current sources (skips if nothing changed)
uv run host-judge

# Force a complete analysis
uv run host-judge --force
```

The analyzer fetches all configured sources, parses and normalizes domains,
applies whitelist filters, calculates deduplication and provenance metrics,
and updates README statistics. It does not generate a combined host file.

### Project Structure

```text
host-judge/
├── src/                     # Application source
│   ├── cli.py               # Main orchestrator (business logic)
│   ├── config.py            # Configuration loading and validation
│   ├── domain_processor.py  # Domain extraction and validation
│   ├── fetcher.py           # HTTP fetching with hash computation
│   ├── pipeline.py          # Deduplication and contribution stats
│   └── state_manager.py     # State persistence and staleness checks
├── tests/                   # Unit and integration tests
├── blocklists.json          # Source configuration
├── whitelist.txt            # Domain whitelist
├── state.json               # Runtime state (hashes, timestamps)
├── pyproject.toml           # Project and tool configuration
├── uv.lock                  # Reproducible dependency lock
└── .pre-commit-config.yaml  # Pre-commit hooks
```

### Development Workflow

**Run tests:**

```bash
# Run all tests
uv run pytest

# Run with coverage report
uv run pytest --cov=src --cov-report=term-missing

# Run specific test file
uv run pytest tests/test_domain_processor.py
```

**Code quality checks:**

```bash
# Linting (with auto-fix)
uv run ruff check src/ tests/ --fix

# Formatting
uv run ruff format src/ tests/

# Type checking
uv run mypy src/

# Run all pre-commit hooks
uv run pre-commit run --all-files

# Audit the hash-locked dependency set
audit_file=$(mktemp)
uv export --locked --all-groups --no-emit-project \
  --format requirements.txt --output-file "$audit_file"
uv run --group audit pip-audit --requirement "$audit_file" \
  --require-hashes --disable-pip --strict
rm -f "$audit_file"
```

### Configuration

Blocklists are configured in `blocklists.json`. Each entry requires a `name` and
`url`; optional fields are `nsfw`, `maintainer_name`, `maintainer_url`, and
`maintainer_description`. Source and maintainer URLs must use public HTTPS
destinations without embedded credentials, fragments, or nonstandard ports.
Source names and URLs must be unique.

Domains can be excluded using `whitelist.txt`. Supports Adblock Plus exception
rules (`@@||domain^`, matches the domain and all subdomains) as well as plain
domains and `*.domain` wildcards.

### Performance Tuning

In `src/cli.py`, adjust:

- `MAX_WORKERS = 5`: Maximum concurrent source fetches
- `MAX_DOMAINS_PER_SOURCE = 10_000_000`: Per-source parsed-domain limit
- `MAX_TOTAL_DOMAINS = 30_000_000`: Aggregate parsed-domain limit

In `src/fetcher.py`, adjust:

- `CONNECT_TIMEOUT_SECONDS = 10`: Connection timeout per attempt
- `READ_TIMEOUT_SECONDS = 90`: Read timeout per attempt
- `MAX_FETCH_ATTEMPTS = 5`: Requests made before a source is reported unavailable
- `MAX_SOURCE_BYTES = 512 MiB`: Maximum decoded body size for one source
- `MAX_TOTAL_DOWNLOAD_BYTES = 2 GiB`: Shared download budget for one analysis
- `MAX_LINE_BYTES = 4 KiB`: Maximum input line length
- `MAX_LINES_PER_SOURCE = 10_000_000`: Per-source input-line limit

In `src/state_manager.py`:

- `STALE_THRESHOLD_DAYS = 30`: Days without an observed content change
  before a source is flagged as a removal candidate

Sources are never removed automatically.

> [!WARNING]
> If you add many sources or experience rate limiting, reduce
> `MAX_WORKERS` to control concurrency.

# Host Judge

Host-list relevance monitor and whitelist workspace. Host Judge measures how
many domains each configured source uniquely contributes and identifies sources
that may no longer justify their inclusion.

The repository-level [`whitelist.txt`](whitelist.txt) is applied before domain
counts and contribution statistics are calculated. The analyzer does not
generate or publish a combined host list.

## Policy

A source is a removal candidate when either condition is true:

- it contributes 1,000 or fewer unique domains to its relevant aggregate; or
- its downloaded content has not changed for at least 30 days.

Sources are never removed automatically. General-list contribution is measured
against the general aggregate; NSFW-list contribution is measured against the
full aggregate.

<!-- STATS_START -->

## Latest run

Last updated: 2026-11-19T23:17:06.149798223Z

- General domains: 5,434,166
- Total domains including NSFW: 10,369,238
- Whitelisted domains excluded: 7

### General host lists

<table>
<!-- markdownlint-disable MD013 -->
<thead><tr><th>Source List</th><th>Total Domains</th><th>Unique Contribution</th><th>Removal Candidate</th></tr></thead>
<tbody>
<tr><td><a href="https://raw.githubusercontent.com/hagezi/nrd/main/adblock/dga30.txt">HaGeZi DGA 30 Days</a></td><td>2,855,322</td><td>2,629,639</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/tif.txt">HaGeZi Threat Intelligence</a></td><td>1,574,390</td><td>1,125,293</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Malware.txt">RPiList Malware</a></td><td>761,608</td><td>572,973</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/AdguardDNS.txt">AdGuard DNS Filter</a></td><td>190,477</td><td>94,113</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/pro.txt">HaGeZi Multi-pro Extended</a></td><td>229,850</td><td>87,693</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/RooneyMcNibNug/pihole-stuff/master/SNAFU.txt">SNAFU</a></td><td>75,967</td><td>73,159</td><td>No</td></tr>
<tr><td><a href="https://lists.cyberhost.uk/malware.txt">Cyber Threat Coalition Malware</a></td><td>101,136</td><td>69,981</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts">Steven Black&#x27;s Unified Hosts</a></td><td>76,105</td><td>22,228</td><td>No</td></tr>
<tr><td><a href="https://big.oisd.nl">OISD Big List</a></td><td>231,743</td><td>20,113</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Easyprivacy.txt">EasyPrivacy</a></td><td>43,261</td><td>12,599</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt">KADhosts</a></td><td>51,586</td><td>11,876</td><td>No</td></tr>
<tr><td><a href="https://hostfiles.frogeye.fr/firstparty-trackers-hosts.txt">First-Party Trackers</a></td><td>15,012</td><td>11,558</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Phishing.txt">RPiList Phishing</a></td><td>153,858</td><td>11,134</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Crypto.txt">Prigent Crypto</a></td><td>11,489</td><td>9,366</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/bigdargon/hostsVN/master/hosts">hostsVN</a></td><td>18,607</td><td>6,248</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/DandelionSprout/adfilt/master/Alternate%20versions%20Anti-Malware%20List/AntiMalwareHosts.txt">DandelionSprout Anti-Malware</a></td><td>12,280</td><td>4,012</td><td>No</td></tr>
<tr><td><a href="https://malware-filter.gitlab.io/malware-filter/phishing-filter-hosts.txt">Phishing Hosts</a></td><td>17,037</td><td>1,337</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Ads.txt">Prigent Ads</a></td><td>4,269</td><td>1,282</td><td>No</td></tr>
</tbody>
<!-- markdownlint-enable MD013 -->
</table>

### NSFW host lists

<table>
<!-- markdownlint-disable MD013 -->
<thead><tr><th>Source List</th><th>Total Domains</th><th>Unique Contribution</th><th>Removal Candidate</th></tr></thead>
<tbody>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Adult.txt">Prigent Adult</a></td><td>4,590,338</td><td>4,431,199</td><td>No</td></tr>
<tr><td><a href="https://nsfw.oisd.nl">OISD NSFW</a></td><td>496,916</td><td>311,946</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/nsfw.txt">HaGeZi NSFW</a></td><td>85,930</td><td>11,698</td><td>No</td></tr>
</tbody>
<!-- markdownlint-enable MD013 -->
</table>

> [!NOTE]
> **Unique Contribution** is the number of domains that would disappear if a
> source were removed. A source is a **Removal Candidate** when it contributes
> 1,000 or fewer unique domains or has no observed content change for 30 days.
> Sources are never removed automatically.

<!-- STATS_END -->

## Local use

Install the pinned Rust toolchain and run:

```bash
cargo run --release --locked -- --force
```

The analyzer retrieves at most five sources concurrently, rejects non-public
destinations and unsafe redirects, bounds input sizes, and uses an external
merge so large lists do not need to fit in memory. It replaces `README.md` and
`state.json` only after every source has been fetched and analyzed successfully.

Install the repository hooks with:

```bash
python -m pip install pre-commit==4.6.2
pre-commit install --install-hooks
```

The hooks enforce formatting, linting, immutable dependency references, secret
scanning, and contribution-metadata policy. Hosted analysis runs weekly and
proposes generated report changes through a pull request.

## Configuration

Sources are declared in [`blocklists.json`](blocklists.json). Each entry requires
`name` and a public HTTPS `url`; it may also set `nsfw` and maintainer metadata.
Exact domains or `*.example.com` suffixes in [`whitelist.txt`](whitelist.txt) are
excluded from all counts.

<div align="center">

# Host Judge

<strong><em>Host-list relevance monitor and whitelist workspace.<br>
Tracks configured sources for staleness, unique contribution, and
removal-candidate analysis.</em></strong>

</div>

## Purpose

Host Judge polls the configured host lists weekly and publishes analytics to this
README. It measures how many domains each source uniquely contributes and flags
sources as removal candidates when they contribute 1,000 or fewer unique domains
or show no observed content change for 30 days.

The repository-level [`whitelist.txt`](whitelist.txt) is applied before domain
counts and contribution statistics are calculated, so the report reflects the
effective policy used for maintained devices.

## How It Works

Runs automatically once a week through GitHub Actions. Sources are fetched with
bounded concurrency, normalized, filtered through the whitelist, and externally
merged for deduplication and provenance analysis. The workflow updates only the
README and analysis state through a pull request; it does not generate or
publish a combined host list.

<!-- STATS_START -->

## Latest Run

<div align="center">

![Total Domains](https://img.shields.io/badge/Total_Domains-10,491,572-ff79c6?style=for-the-badge&labelColor=6272a4)
![Last Updated](https://img.shields.io/badge/Last_Updated-2026--11--22_10:31:20_UTC-50fa7b?style=for-the-badge&labelColor=6272a4)

### General Host Lists

<table align="center">
<!-- markdownlint-disable MD013 -->
<thead><tr><th>Source List</th><th>Total Domains</th><th>Unique Contribution</th><th>Removal Candidate</th></tr></thead>
<tbody>
<tr><td><a href="https://raw.githubusercontent.com/hagezi/nrd/main/adblock/dga30.txt">HaGeZi DGA 30 Days</a></td><td>2,660,503</td><td>2,555,974</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/tif.txt">HaGeZi Threat Intelligence</a></td><td>1,865,694</td><td>1,478,532</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Malware.txt">RPiList Malware</a></td><td>761,608</td><td>534,859</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/AdguardDNS.txt">AdGuard DNS Filter</a></td><td>190,477</td><td>93,719</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/pro.txt">HaGeZi Multi-pro Extended</a></td><td>283,996</td><td>79,747</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/RooneyMcNibNug/pihole-stuff/master/SNAFU.txt">SNAFU</a></td><td>75,969</td><td>73,151</td><td>No</td></tr>
<tr><td><a href="https://lists.cyberhost.uk/malware.txt">Cyber Threat Coalition Malware</a></td><td>100,656</td><td>69,095</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts">Steven Black&#x27;s Unified Hosts</a></td><td>76,105</td><td>19,819</td><td>No</td></tr>
<tr><td><a href="https://big.oisd.nl">OISD Big List</a></td><td>224,387</td><td>19,611</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/PolishFiltersTeam/KADhosts/master/KADhosts.txt">KADhosts</a></td><td>51,251</td><td>13,800</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Easyprivacy.txt">EasyPrivacy</a></td><td>43,261</td><td>12,585</td><td>No</td></tr>
<tr><td><a href="https://hostfiles.frogeye.fr/firstparty-trackers-hosts.txt">First-Party Trackers</a></td><td>15,012</td><td>11,513</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/RPiList-Phishing.txt">RPiList Phishing</a></td><td>153,858</td><td>11,038</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Crypto.txt">Prigent Crypto</a></td><td>11,489</td><td>9,336</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/bigdargon/hostsVN/master/hosts">hostsVN</a></td><td>18,607</td><td>6,218</td><td>No</td></tr>
<tr><td><a href="https://raw.githubusercontent.com/DandelionSprout/adfilt/master/Alternate%20versions%20Anti-Malware%20List/AntiMalwareHosts.txt">DandelionSprout Anti-Malware</a></td><td>12,279</td><td>4,165</td><td>No</td></tr>
<tr><td><a href="https://malware-filter.gitlab.io/malware-filter/phishing-filter-hosts.txt">Phishing Hosts</a></td><td>16,647</td><td>1,488</td><td>No</td></tr>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Ads.txt">Prigent Ads</a></td><td>4,269</td><td>1,283</td><td>No</td></tr>
</tbody>
<!-- markdownlint-enable MD013 -->
</table>

### NSFW Host Lists

<table align="center">
<!-- markdownlint-disable MD013 -->
<thead><tr><th>Source List</th><th>Total Domains</th><th>Unique Contribution</th><th>Removal Candidate</th></tr></thead>
<tbody>
<tr><td><a href="https://v.firebog.net/hosts/Prigent-Adult.txt">Prigent Adult</a></td><td>4,590,338</td><td>4,428,162</td><td>No</td></tr>
<tr><td><a href="https://nsfw.oisd.nl">OISD NSFW</a></td><td>498,501</td><td>280,411</td><td>No</td></tr>
<tr><td><a href="https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/adblock/nsfw.txt">HaGeZi NSFW</a></td><td>87,403</td><td>12,524</td><td>No</td></tr>
</tbody>
<!-- markdownlint-enable MD013 -->
</table>

</div>

> [!NOTE]
> **Unique Contribution** is the number of domains that would disappear if a
> source were removed. A source is a **Removal Candidate** when it contributes
> 1,000 or fewer unique domains or has no observed content change for 30 days.
> Sources are never removed automatically.

<!-- STATS_END -->

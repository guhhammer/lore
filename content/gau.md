# gau

Get All URLs: fetch known URLs from AlienVault's Open Threat Exchange, the Wayback Machine, and Common Crawl for any domains.

## Usage
- `gau example.com` : fetch all URLs of a domain from AlienVault's Open Threat Exchange, the Wayback Machine, Common Crawl, and URLScan
- `gau domain1 domain2 ...` : fetch URLs of multiple domains
- `gau <path/to/domains.txt --threads 4` : fetch all URLs of several domains from an input file, running multiple threads
- `gau example.com --o path/to/found_urls.txt` : write output results to a file
- `gau --providers wayback|commoncrawl|otx|urlscan example.com` : search for URLs from only one specific provider
- `gau --providers wayback,otx,... example.com` : search for URLs from multiple providers
- `gau --from YYYYMM --to YYYYMM example.com` : search for URLs within specific date range

## Categories
files

## Links
- More information: https://github.com/lc/gau#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gau

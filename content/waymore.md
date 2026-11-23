# waymore

Fetch URLs of a domain from Wayback Machine, Common Crawl, Alien Vault OTX, URLScan, and VirusTotal. Note: Unless specified, output is dumped into the `results/` directory where waymore's `config.yml` resides (by default in `~/.config/waymore/`).

## Usage
- `waymore -i example.com` : search for URLs of a domain (output will typically be in `~/.config/waymore/results/`)
- `waymore -mode U -oU path/to/example.com-urls.txt -i example.com` : limit search results to only include a list of URLs for a domain and store outputs to the specified file
- `waymore -mode R -oR path/to/example.com-url-responses -i example.com` : only output the content bodies of URLs and store outputs to the specified directory
- `waymore -from YYYYMMDD|YYYYMM|YYYY -to YYYYMMDD|YYYYMM|YYYY -i example.com` : filter the results by specifying date ranges

## Categories
files, network

## Links
- More information: https://github.com/xnl-h4ck3r/waymore
- tldr page: https://tldr.inbrowser.app/pages/common/waymore

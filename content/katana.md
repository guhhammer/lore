# katana

A fast crawler focused on execution in automation pipelines offering both headless and non-headless crawling.

## Usage
- `katana -list https://example.com,https://google.com,...` : crawl a list of URLs
- `katana -u https://example.com -hl` : crawl a uRL using headless mode using Chromium
- `katana -proxy http://127.0.0.1:8080 -H path/to/headers.txt -u https://example.com` : pass requests through a proxy (http/socks5) and use custom headers from a file
- `katana -s depth-first|breadth-first -d value -rl value -u https://example.com` : specify the crawling strategy, depth of subdirectories to crawl, and rate limiting (requests per second)
- `subfinder -dL path/to/domains.txt | katana -ct value -o path/to/output.txt` : find subdomains using `subfinder`, crawl each for a maximum number of seconds, and write results to an output file

## See also
- gau
- scrapy
- waymore

## Categories
network, files

## Links
- More information: https://docs.projectdiscovery.io/opensource/katana/usage
- tldr page: https://tldr.inbrowser.app/pages/common/katana

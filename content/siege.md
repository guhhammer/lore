# siege

HTTP loadtesting and benchmarking tool.

## Usage
- `siege https://example.com` : test a URL with default settings
- `siege -f path/to/url_list.txt` : test a list of URLs
- `siege -i -f path/to/url_list.txt` : test list of URLs in a random order (Simulates internet traffic)
- `siege -b -f path/to/url_list.txt` : benchmark a list of URLs (without waiting between requests)
- `siege -c 50 -f path/to/url_list.txt` : set the amount of concurrent connections
- `siege -t 30s -f path/to/url_list.txt` : set how long for the siege to run for

## Categories
network, files, development

## Links
- More information: https://www.joedog.org/siege/manual/
- tldr page: https://tldr.inbrowser.app/pages/common/siege

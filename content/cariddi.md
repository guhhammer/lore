# cariddi

Crawl URLs and scan for endpoints, secrets, api keys, file extensions, tokens, and more from a list of domains.

## Usage
- `cat path/to/urls.txt | cariddi -s -sf path/to/custom_secrets.txt -json` : hunt for secrets using custom `regex`es and output results in JSON
- `cat path/to/urls.txt | cariddi -e -c 250 -t 15 -plain` : hunt for juicy endpoints with high concurrency and timeout with plain output results
- `cat path/to/urls.txt | cariddi -debug -sr -ot path/to/debug_output.txt` : crawl with debug mode and store HTTP responses and output results in `txt` file
- `cat path/to/urls.txt | cariddi -intensive -proxy http://127.0.0.1:8080 -rua -oh path/to/intensive_crawl.html` : perform an intensive crawl with a proxy and random user agent and output results in `html` file
- `cat path/to/urls.txt | cariddi -err -info -d 3 -cache` : hunt for errors and useful information with a custom delay and use `.cariddi_cache` folder as cache
- `cariddi -examples` : show example uses

## Categories
files, network, text

## Links
- More information: https://github.com/edoardottt/cariddi/wiki
- tldr page: https://tldr.inbrowser.app/pages/common/cariddi

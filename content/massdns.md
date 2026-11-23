# massdns

Resolve DNS records in bulk with high performance for reconnaissance.

## Usage
- `massdns -r path/to/resolvers.txt path/to/domains.txt` : resolve `A` records for domains in a file using specified resolvers
- `massdns -r path/to/resolvers.txt -t A|AAAA|NS|MX|TXT -w path/to/output.txt path/to/domains.txt` : resolve a specific record type and write results to a file
- `massdns -r path/to/resolvers.txt -o S path/to/domains.txt` : resolve domains with simple text output format
- `massdns -r path/to/resolvers.txt -o J path/to/domains.txt` : resolve domains with JSON output format
- `massdns -r path/to/resolvers.txt -s 10000 path/to/domains.txt` : resolve domains with a custom number of concurrent lookups
- `massdns -r path/to/resolvers.txt -q path/to/domains.txt` : resolve domains in quiet mode, suppressing status output

## See also
- dig
- dnsx

## Categories
files, network, monitoring

## Links
- More information: https://github.com/blechschmidt/massdns#usage
- tldr page: https://tldr.inbrowser.app/pages/common/massdns

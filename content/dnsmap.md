# dnsmap

Scan a domain for common subdomains e.g. smtp.domain.org.

## Usage
- `dnsmap example.com` : scan for subdomains using the internal wordlist
- `dnsmap example.com -w path/to/wordlist.txt` : specify a list of subdomains to check for
- `dnsmap example.com -c path/to/file.csv` : store results to a CSV file
- `dnsmap example.com -i 123.45.67.89,98.76.54.32` : ignore 2 IPs that are false positives (up to 5 possible)

## Categories
files

## Links
- More information: https://github.com/resurrecting-open-source-projects/dnsmap
- tldr page: https://tldr.inbrowser.app/pages/linux/dnsmap

# dnsrecon

DNS enumeration tool.

## Usage
- `dnsrecon -d example.com --db path/to/database.sqlite` : scan a domain and save the results to an SQLite database
- `dnsrecon -d example.com -n nameserver.example.com -t axfr` : scan a domain, specifying the nameserver and performing a zone transfer
- `dnsrecon -d example.com -D path/to/dictionary.txt -t brt` : scan a domain, using a brute-force attack and a dictionary of subdomains and hostnames
- `dnsrecon -d example.com -s -j` : scan a domain, performing a reverse lookup of IP ranges from the SPF record and saving the results to a JSON file
- `dnsrecon -d example.com -g -c` : scan a domain, performing a Google enumeration and saving the results to a CSV file
- `dnsrecon -d example.com -t snoop -n nameserver.example.com -D path/to/dictionary.txt` : scan a domain, performing DNS cache snooping
- `dnsrecon -d example.com -t zonewalk` : scan a domain, performing zone walking

## Categories
network, files, databases

## Links
- More information: https://manned.org/dnsrecon
- tldr page: https://tldr.inbrowser.app/pages/linux/dnsrecon

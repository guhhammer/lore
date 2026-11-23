# dnsx

A fast and multi-purpose DNS toolkit to run multiple DNS queries. Note: Input to `dnsx` needs to be passed through `stdin` (pipe `|`) in some cases.

## Usage
- `echo example.com | dnsx -a -re` : query the A record of a (sub)domain and show [re]sponse received
- `dnsx <<<example.com -recon -re` : query all the DNS records (A, AAAA, CNAME, NS, TXT, SRV, PTR, MX, SOA, AXFR, CAA)
- `echo example.com | dnsx -re -a|aaaa|cname|ns|txt|srv|ptr|mx|soa|any|axfr|caa` : query a specific type of DNS record
- `echo example.com | dnsx -ro` : output response only (do not show the queried domain or subdomain)
- `echo example.com | dnsx -debug|raw -r 1.1.1.1,8.8.8.8,... -retry number` : display raw response of a query, specifying resolvers to use and retry attempts for failures
- `dnsx -d FUZZ.example.com -w path/to/wordlist.txt -re` : brute force DNS records using a placeholder
- `dnsx -d path/to/domain.txt -w path/to/wordlist.txt -re -o path/to/output.txt -nc` : brute force DNS records from a list of domains and wordlists, appending output to a file with no color codes
- `subfinder -silent -d example.com | dnsx -cname -re -rl number` : extract `CNAME` records for the given list of subdomains, with rate limiting DNS queries per second

## See also
- dig
- dog
- dnstracer

## Categories
network, files, databases

## Links
- More information: https://docs.projectdiscovery.io/opensource/dnsx/usage
- tldr page: https://tldr.inbrowser.app/pages/common/dnsx

# drill

Perform various DNS queries.

## Usage
- `drill example.com` : look up the IP(s) associated with a hostname (A records)
- `drill mx example.com` : look up the mail server(s) associated with a given domain name (MX record)
- `drill any example.com` : get all types of records for a given domain name
- `drill example.com @8.8.8.8` : specify an alternate DNS server to query
- `drill -x 8.8.8.8` : perform a reverse DNS lookup on an IP address (PTR record)
- `drill -TD example.com` : perform DNSSEC trace from root servers down to a domain name
- `drill -s dnskey example.com` : show DNSKEY record(s) for a domain name

## Categories
network, media

## Links
- More information: https://manned.org/drill
- tldr page: https://tldr.inbrowser.app/pages/common/drill

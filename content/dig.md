# dig

DNS lookup utility.

## Usage
- `dig +short example.com` : look up the IP(s) associated with a hostname (A records)
- `dig +noall +answer example.com` : get a detailed answer for a given domain (A records)
- `dig +short example.com A|MX|TXT|CNAME|NS` : query a specific DNS record type associated with a given domain name
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : specify an alternate DNS server to query and optionally use DNS over TLS (DoT)
- `dig -x 8.8.8.8` : perform a reverse DNS lookup on an IP address (PTR record)
- `dig +nssearch example.com` : find authoritative name servers for the zone and display SOA records
- `dig +trace example.com` : perform iterative queries and display the entire trace path to resolve a domain name
- `dig +tcp -p port @dns_server_ip example.com` : query a DNS server over a non-standard port using the TCP protocol

## See also
- resolvectl
- nslookup
- host

## Categories
network

## Links
- More information: https://manned.org/dig
- tldr page: https://tldr.inbrowser.app/pages/common/dig
- Wikipedia: https://en.wikipedia.org/wiki/Dig_(command)

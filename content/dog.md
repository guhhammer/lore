# dog

DNS lookup utility. It has colorful output, supports DNS-over-TLS and DNS-over-HTTPS protocols, and can emit JSON.

## Usage
- `dog example.com` : look up the IP(s) associated with a hostname (A records)
- `dog example.com MX` : query the MX records type associated with a given domain name
- `dog example.com MX @1.1.1.1` : specify a specific DNS server to query (e.g. Cloudflare)
- `dog example.com MX @1.1.1.1 -T` : query over TCP rather than UDP
- `dog -q example.com -t MX -n 1.1.1.1 -T` : query the MX records type associated with a given domain name over TCP using explicit arguments
- `dog example.com -H @https://cloudflare-dns.com/dns-query` : look up the IP(s) associated with a hostname (A records) using DNS over HTTPS (DoH)

## Categories
network, databases, security

## Links
- More information: https://github.com/ogham/dog#examples
- tldr page: https://tldr.inbrowser.app/pages/common/dog

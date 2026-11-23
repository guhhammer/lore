# kdig

Advanced DNS lookup utility.

## Usage
- `kdig example.com` : look up the IP(s) associated with a hostname (A records)
- `kdig example.com @8.8.8.8` : specify a specific DNS server to query (e.g. Google DNS)
- `kdig example.com A|AAAA|NS|SOA|DNSKEY|ANY` : query a specific DNS record type associated with a given domain name
- `kdig -d @8.8.8.8 +tls-ca +tls-host=dns.google example.com` : look up the IP(s) associated with a hostname (A records) using DNS over TLS (DoT)
- `kdig -d @1.1.1.1 +https +tls-hostname=1dot1dot1dot1.cloudflare-dns.com example.com` : look up the IP(s) associated with a hostname (A records) using DNS over HTTPS (DoH)

## Categories
network, security

## Links
- More information: https://www.knot-dns.cz/docs/latest/html/man_kdig.html
- tldr page: https://tldr.inbrowser.app/pages/common/kdig

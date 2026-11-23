# knotc

Control knot DNS server.

## Usage
- `knotc zone-begin zone` : start editing a zone
- `knotc zone-set zone subdomain 3600 A ip_address` : set an A record with TTL of 3600
- `knotc zone-commit zone` : finish editing the zone
- `knotc zone-read zone` : get the current zone data
- `knotc conf-read server` : get the current server configuration

## Categories
network

## Links
- More information: https://www.knot-dns.cz/docs/latest/html/man_knotc.html
- tldr page: https://tldr.inbrowser.app/pages/common/knotc

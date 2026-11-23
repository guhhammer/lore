# cli4

Python command-line interface for Cloudflare API.

## Usage
- `cli4 /user` : display account information
- `cli4 /zones` : list all zones
- `cli4 /zones/:example.com/dns_records` : list DNS records for a specific zone
- `cli4 --post name=example.com type=A content=192.0.2.1 /zones/:example.com/dns_records` : create a new DNS record
- `cli4 --put name=sub.example.com type=A content=192.0.2.2 /zones/:example.com/dns_records/:record_id` : update an existing DNS record
- `cli4 --delete /zones/:example.com/dns_records/:record_id` : delete a DNS record
- `cli4 --post purge_everything=true /zones/:example.com/purge_cache` : purge all cache for a zone

## Categories
network, media, text

## Links
- More information: https://github.com/cloudflare/python-cloudflare
- tldr page: https://tldr.inbrowser.app/pages/common/cli4

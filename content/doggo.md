# doggo

DNS client for Humans. Written in Golang.

## Usage
- `doggo example.com` : perform a simple DNS lookup
- `doggo MX codeberg.org @1.1.1.2` : query MX records using a specific nameserver
- `doggo example.com @https://dns.quad9.net/dns-query` : use DNS over HTTPS
- `doggo example.com -J | jq '.responses[0].answers[].address'` : output in the JSON format
- `doggo -x 8.8.4.4 --short` : perform a reverse DNS lookup

## Categories
network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/doggo

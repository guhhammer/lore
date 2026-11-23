# flarectl

Official CLI for Cloudflare.

## Usage
- `flarectl firewall rules create --zone="example.com" --value="8.8.8.8" --mode="block" --notes="Block bad actor"` : block a specific IP
- `flarectl dns create --zone="example.com" --name="app" --type="CNAME" --content="myapp.herokuapp.com" --proxy` : add a DNS record
- `flarectl ips --ip-type ipv4|ipv6|all` : list all Cloudflare IPv4/IPv6 ranges
- `for domain in $(cat domains.txt); do flarectl zone info --zone=$domain; done` : create many new Cloudflare zones automatically with names from `domains.txt`
- `flarectl firewall rules list` : list all firewall rules

## Categories
network, security

## Links
- More information: https://github.com/cloudflare/cloudflare-go/blob/v0/cmd/flarectl/README.md
- tldr page: https://tldr.inbrowser.app/pages/common/flarectl

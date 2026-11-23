# subfinder

Discover valid subdomains for websites. Designed as a passive framework to be useful for bug bounties and safe for penetration testing.

## Usage
- `subfinder -d example.com` : find subdomains for a specific domain
- `subfinder -silent -d example.com` : show only the subdomains found
- `subfinder -nW -d example.com` : show only active subdomains
- `subfinder -all -d example.com` : use all sources for enumeration
- `subfinder -r 8.8.8.8,1.1.1.1,... -d example.com` : use a given comma-separated list of resolvers

## Links
- More information: https://docs.projectdiscovery.io/opensource/subfinder/usage
- tldr page: https://tldr.inbrowser.app/pages/common/subfinder

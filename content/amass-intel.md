# amass intel

Collect open source intel on an organization like root domains and ASNs.

## Usage
- `amass intel -addr 192.168.0.1-254` : find root domains in an IP [addr]ess range
- `amass intel -active -addr 192.168.0.1-254` : use active recon methods
- `amass intel -whois -d domain_name` : find root domains related to a domain
- `amass intel -org organization_name` : find ASNs belonging to an [org]anization
- `amass intel -asn asn` : find root domains belonging to a given Autonomous System Number
- `amass intel -o output_file -whois -d domain_name` : save results to a text file
- `amass intel -list` : list all available data sources

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/amass-intel

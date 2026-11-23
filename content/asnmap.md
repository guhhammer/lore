# asnmap

A Go CLI tool for mapping organization network ranges using ASN information. Note: An API key is required from ProjectDiscovery Cloud Platform for the tool to work.

## Usage
- `asnmap -a AS5650 -silent` : look up CIDR ranges for an ASN
- `asnmap -i 100.19.12.21 -silent` : look up CIDR ranges for an IP address
- `asnmap -d example.com -silent` : look up CIDR ranges for a domain
- `asnmap -org GOOGLE -silent` : look up CIDR ranges for an organization
- `asnmap -f path/to/targets.txt -silent` : look up CIDR ranges from a file of targets
- `asnmap -d facebook.com -j -silent` : output results in JSON format
- `asnmap -a AS394161 -c -silent` : output results in CSV format
- `asnmap -up` : update asnmap to the latest version

## Categories
network, cloud

## Links
- More information: https://github.com/projectdiscovery/asnmap#usage
- tldr page: https://tldr.inbrowser.app/pages/common/asnmap

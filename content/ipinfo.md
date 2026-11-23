# ipinfo

Official CLI for the IPinfo.io IP geolocation and network intelligence API. Note: Some commands will require a token from IPinfo.io.

## Usage
- `ipinfo myip` : display details for your current IP address
- `ipinfo ip_address` : display details for a specific IP address
- `ipinfo bulk path/to/ips.txt` : display details for multiple IP addresses in bulk from a file
- `ipinfo bulk cidr_range` : display details for a CIDR or IP range
- `ipinfo ip_address -f hostname,country,org` : display only specific fields from IP lookup results
- `ipinfo summarize path/to/ips.txt` : summarize details for a group of IP addresses
- `ipinfo grepip path/to/file.txt` : extract IP addresses from text and highlight them
- `ipinfo -h` : display help

## Categories
network, desktop, files

## Links
- More information: https://github.com/ipinfo/cli#quick-start
- tldr page: https://tldr.inbrowser.app/pages/common/ipinfo

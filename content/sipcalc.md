# sipcalc

Calculate IP subnet information for IPv4 and IPv6 addresses.

## Usage
- `sipcalc 192.168.1.0/24` : display subnet information for an IPv4 CIDR address
- `sipcalc -a 192.168.1.0/24` : display all available information for an address
- `sipcalc -s 255.255.255.128 192.168.1.0/24` : split an IPv4 network into subnets of a specified mask
- `sipcalc -r 2001:db8::/32` : display IPv6 reverse DNS information
- `sipcalc -d 192.168.1.0/24` : enable DNS name resolution
- `sipcalc -h` : display help

## See also
- ipcalc

## Categories
network, desktop

## Links
- More information: https://manned.org/sipcalc
- tldr page: https://tldr.inbrowser.app/pages/common/sipcalc

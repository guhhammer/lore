# ipset

Create IP sets for firewall rules.

## Usage
- `ipset create set_name hash:ip` : create an empty IP set which will contain IP addresses
- `ipset destroy set_name` : destroy a specific IP set
- `ipset add set_name 192.168.1.25` : add an IP address to a specific set
- `ipset del set_name 192.168.1.25` : delete a specific IP address from a set
- `ipset save set_name >path/to/ip_set` : save an IP set

## Categories
network, security

## Links
- More information: https://manned.org/ipset
- tldr page: https://tldr.inbrowser.app/pages/linux/ipset

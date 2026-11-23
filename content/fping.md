# fping

A more powerful ping which can ping multiple hosts.

## Usage
- `fping 192.168.1.{1..254}` : list the status of all hosts within a range
- `fping -a -g 192.168.1.0/24` : list alive hosts within a subnet generated from a netmask
- `fping -q -a -g 192.168.1.1 192.168.1.254` : list alive hosts within a subnet generated from an IP range and prune per-probe results
- `fping -u -g 192.168.1.0/24` : list unreachable hosts within a subnet generated from a netmask

## Categories
network

## Links
- More information: https://fping.org/fping.8.html
- tldr page: https://tldr.inbrowser.app/pages/common/fping

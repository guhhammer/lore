# cidr

Simplify IPv4/IPv6 CIDR network prefix management with counting, overlap checking, explanation, and subdivision.

## Usage
- `cidr explain 10.0.0.0/16` : explain a CIDR range
- `cidr contains 10.0.0.0/16 10.0.14.5` : check whether an address belongs to a CIDR range
- `cidr count 10.0.0.0/16` : get a count of all addresses in a CIDR range
- `cidr overlaps 10.0.0.0/16 10.0.14.0/22` : check whether two CIDR ranges overlap
- `cidr divide 10.0.0.0/16 9` : divide a CIDR range into a specific number of networks

## Categories
network

## Links
- More information: https://github.com/bschaatsbergen/cidr
- tldr page: https://tldr.inbrowser.app/pages/common/cidr

# mtr

Matt's Traceroute: combined traceroute and ping tool.

## Usage
- `mtr example.com` : traceroute to a host and continuously ping all intermediary hops
- `mtr -n example.com` : disable IP address and host name mapping
- `mtr -w example.com` : generate output after pinging each hop 10 times
- `mtr -4 example.com` : force IPv4 or IPv6
- `mtr -i 10 example.com` : wait for a given time (in seconds) before sending another packet to the same hop
- `mtr -z example.com` : display the Autonomous System Number (ASN) for each hop
- `mtr -b example.com` : display both IP address and reverse DNS name

## See also
- traceroute
- ping

## Categories
network

## Links
- More information: https://manned.org/mtr
- tldr page: https://tldr.inbrowser.app/pages/common/mtr

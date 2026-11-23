# gtraceroute

This command is an alias of GNU `traceroute`.

## Usage
- `traceroute example.com` : traceroute to a host
- `traceroute -n example.com` : disable IP address and host name mapping
- `traceroute -w 0.5 example.com` : specify wait time in seconds for response
- `traceroute -q 5 example.com` : specify number of queries per hop
- `traceroute example.com 42` : specify size in bytes of probing packet
- `traceroute --mtu example.com` : determine the MTU to the destination
- `traceroute -I example.com` : use ICMP instead of UDP for tracerouting

## Categories
network, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gtraceroute

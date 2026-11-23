# ip route list

Display subcommand for IP Routing table management.

## Usage
- `ip r l` : display the `main` routing table
- `ip r l t main|254` : display the main routing table (same as first example)
- `ip r l t local|255` : display the local routing table
- `ip r l t all|unspec|0` : display all routing tables
- `ip r l dev ethX` : list routes from a given device only
- `ip r l s link` : list routes within a given scope
- `ip r l c` : display the routing cache
- `ip -6|-4 r` : display only IPv6 or IPv4 routes

## Categories
desktop, databases

## Links
- More information: https://manned.org/ip-route
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-route-list

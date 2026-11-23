# traceroute6

Shows the path IPv6 packets take to a host, one router (hop) per line, with the time each hop takes.

## Usage
- `traceroute6 example.com` : trace the route to a host over IPv6
- `traceroute6 -n example.com` : do not resolve names, which is faster
- `traceroute6 -m 20 example.com` : give up after 20 hops
- `traceroute6 -I example.com` : probe with ICMP echo packets instead of UDP

## Notes
Lines of stars mean a router did not answer. That is common and does not by itself mean a problem.

## Categories
network, text, files

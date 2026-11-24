# ip

Show/manipulate routing, devices, policy routing, and tunnels. Some subcommands such as `address` have their own usage documentation.

## Usage
- `ip a` : list interfaces with detailed info
- `ip -br a` : list interfaces with brief network layer info
- `ip -br l` : list interfaces with brief link layer info
- `ip r` : display the routing table
- `ip n` : show neighbors (ARP table)
- `sudo ip l s ethX up|down` : make an interface up/down
- `sudo ip a add|delete ip_address/mask dev ethX` : add/Delete an IP address to an interface
- `sudo ip r a default via ip_address dev ethX` : add a default route

## Categories
network

## Links
- More information: https://manned.org/ip.8
- tldr page: https://tldr.inbrowser.app/pages/linux/ip
- Wikipedia: https://en.wikipedia.org/wiki/Iproute2

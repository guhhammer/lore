# ip neighbor

Neighbor/ARP/NDP tables management IP subcommand.

## Usage
- `ip n` : display the neighbor/ARP table entries
- `sudo ip n f dev ethX` : remove entries in the neighbor table on device `ethX`
- `ip n g lookup_ip dev ethX` : perform a neighbor lookup and return a neighbor entry
- `sudo ip n add|delete ip_address lladdr mac_address dev ethX nud reachable` : add or delete an ARP entry for the neighbor IP address to `ethX`
- `sudo ip n change|replace ip_address lladdr new_mac_address dev ethX` : change or replace an ARP entry for the neighbor IP address to `ethX`
- `ip -6|-4 n` : display only IPv6 or IPv4 neighbors

## See also
- arp-scan

## Categories
network, users

## Links
- More information: https://manned.org/ip-neighbour
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-neighbor

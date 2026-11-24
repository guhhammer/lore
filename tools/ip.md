# ip

ip shows and changes the network setup of a Linux machine: addresses, links, routes and neighbours. It replaces the old ifconfig and route.

## Tasks
### Look at the network
- `ip addr` : list every interface with its addresses
- `ip -br addr` : the same in one short line per interface
- `ip -4 addr show dev NAME` : only the IPv4 addresses of one interface
- `ip link` : list interfaces and whether they are up
- `ip -s link show NAME` : packet and error counters of one interface
### Addresses
- `sudo ip addr add 192.168.1.50/24 dev NAME` : add an address (lost at reboot)
- `sudo ip addr del 192.168.1.50/24 dev NAME` : remove an address
- `sudo ip addr flush dev NAME` : remove every address of an interface
### Interfaces
- `sudo ip link set NAME up` : turn an interface on
- `sudo ip link set NAME down` : turn an interface off
- `sudo ip link set NAME mtu 1400` : change the MTU
- `sudo ip link add br0 type bridge` : make a bridge
- `sudo ip link delete NAME` : delete a virtual interface
### Routes
- `ip route` : show the routing table
- `ip route get 8.8.8.8` : show which route and source address a destination would use
- `sudo ip route add default via 192.168.1.1` : set the default gateway
- `sudo ip route add 10.0.0.0/8 via 192.168.1.254` : send a network through a gateway
- `sudo ip route del 10.0.0.0/8` : remove a route
### Neighbours and more
- `ip neigh` : show the ARP table (who is on the local network)
- `sudo ip neigh flush all` : forget every neighbour entry
- `ip -j addr` : print the answer as JSON, for scripts
- `ip rule` : show the policy routing rules
- `ip netns list` : list network namespaces

## Notes
- Changes made with ip are lost at reboot: put lasting ones in NetworkManager, netplan or systemd-networkd.
- Taking down the interface you are connected through by ssh cuts you off.
- Most commands also take a short form: `ip a`, `ip l`, `ip r`.
- `ip -c` colours the output.

## See also
- ss
- ping
- nmcli
- traceroute

## Links
- Manual: https://man7.org/linux/man-pages/man8/ip.8.html

## Categories
network system

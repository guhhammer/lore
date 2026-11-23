# ip address

IP Address management subcommand.

## Usage
- `ip a` : list network interfaces and their associated IP addresses
- `ip a s up` : filter to show only active network interfaces
- `ip a s ethX` : display information about a specific network interface
- `sudo ip a a ip_address dev ethX` : add an IP address to a network interface
- `sudo ip a d ip_address dev ethX` : remove an IP address from a network interface
- `sudo ip a f ethX scope global|host|link` : delete all IP addresses in a given scope from a network interface

## Categories
network, users

## Links
- More information: https://manned.org/ip-address
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-address

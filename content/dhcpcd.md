# dhcpcd

DHCP client.

## Usage
- `sudo dhcpcd -k` : release all address leases
- `sudo dhcpcd -n` : request the DHCP server for new leases
- `sudo dhcpcd -U interface_name` : print (dump) the last acquired lease for a given interface
- `sudo dhcpcd -U` : print the last acquired lease for all interfaces

## Categories
users, network

## Links
- More information: https://roy.marples.name/projects/dhcpcd
- tldr page: https://tldr.inbrowser.app/pages/linux/dhcpcd

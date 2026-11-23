# ip link

Manage network interfaces.

## Usage
- `ip l` : show information about all network interfaces
- `ip l sh ethX` : show information about a specific network interface
- `sudo ip l s ethX up|down` : bring a network interface up or down
- `sudo ip l s ethX al "LAN Interface"` : give a meaningful name to a network interface
- `sudo ip l s ethX a ff:ff:ff:ff:ff:ff` : change the MAC address of a network interface
- `sudo ip l s ethX mtu 9000` : change the MTU size for a network interface to use jumbo frames
- `sudo ip l s ethX promisc on|off` : set the promisc mode status of a device
- `sudo ip l d ethX` : delete a device

## Categories
network, users, files

## Links
- More information: https://manned.org/ip-link
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-link

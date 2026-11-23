# arping

Discover and probe hosts in a network using the ARP protocol. Useful for MAC address discovery.

## Usage
- `sudo arping host_ip` : ping a host by ARP request packets
- `sudo arping -I interface host_ip` : ping a host on a specific Interface
- `sudo arping -f host_ip` : ping a host and finish after the first reply
- `sudo arping -c count host_ip` : ping a host a specific number (count) of times
- `sudo arping -U ip_to_broadcast` : broadcast ARP request packets to update neighbors' ARP caches (Unsolicited ARP mode)
- `sudo arping -D -w 3 ip_to_check` : detect duplicated IP addresses in the network by sending ARP requests with a 3 second timeout

## Categories
network, users

## Links
- More information: https://manned.org/arping
- tldr page: https://tldr.inbrowser.app/pages/common/arping

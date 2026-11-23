# ip route

IP Routing table management subcommand.

## Usage
- `ip r` : display the `main` routing table
- `sudo ip r a default via gateway_ip` : add a default route using gateway forwarding
- `sudo ip r a default dev ethX` : add a default route using `ethX`
- `sudo ip r a destination_ip via gateway_ip dev ethX` : add a static route
- `sudo ip r d destination_ip dev ethX` : delete a static route
- `sudo ip r change|replace destination_ip via gateway_ip dev ethX` : change or replace a static route
- `ip r g destination_ip` : show which route will be used by the kernel to reach an IP address
- `ip r l t table_number` : display a specific routing table

## See also
- routel
- ip rule

## Categories
databases, users, network

## Links
- More information: https://manned.org/ip-route
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-route

# ip route add

Add a new networking route.

## Usage
- `sudo ip r a default via gateway_ip` : add a default route using gateway forwarding
- `sudo ip r a default dev ethX` : add a default route using `ethX`
- `sudo ip r a destination_ip via gateway_ip dev ethX` : add a static route
- `sudo ip r a destination_ip dev ethX t ip_address` : add a route to a specific routing table
- `sudo ip r a default via gateway_ip dev ethX table table_id` : force internet traffic to go through a specific device while leaving local traffic unchanged (requires also creating a rule)

## Categories
users

## Links
- More information: https://manned.org/ip-route
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-route-add

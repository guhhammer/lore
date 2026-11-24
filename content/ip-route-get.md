# ip route get

Get a single route to a destination and print its contents exactly as the kernel sees it.

## Usage
- `ip r g 1.1.1.1` : print route to a destination
- `ip r g destination from source` : print route to a destination from a specific source address
- `ip r g destination iif ethX` : print route to a destination for packets arriving on a specific interface
- `ip r g destination oif ethX` : print route to a destination, forcing output through a specific interface
- `ip r g destination tos 0x10` : print route to a destination with a specified Type of Service (ToS)
- `ip r g destination vrf myvrf` : print route to a destination using a specific VRF (Virtual Routing and Forwarding) instance

## Categories
documents, network, system

## Links
- More information: https://manned.org/ip-route
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-route-get
- Wikipedia: https://en.wikipedia.org/wiki/Iproute2

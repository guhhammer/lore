# xdp-filter

Load and manage an eBPF XDP packet filter. Part of the xdp-tools collection.

## Usage
- `sudo xdp-filter load -p allow -m skb network_interface` : load the filter on an interface in skb (generic) mode with default allow policy
- `sudo xdp-filter unload network_interface` : unload the filter from an interface
- `sudo xdp-filter port destination_port` : deny traffic to a specific destination port
- `sudo xdp-filter ip -m src source_ip` : deny traffic from a specific source IP address
- `sudo xdp-filter ether -m src mac_address` : deny traffic from a specific source MAC address
- `sudo xdp-filter poll -i 10000` : poll packets and show statistics every 10000 milliseconds

## Categories
network, monitoring, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/xdp-filter

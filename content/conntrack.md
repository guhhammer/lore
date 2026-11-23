# conntrack

Interact with the Netfilter connection tracking system. Search, list, inspect, modify, and delete connection flows.

## Usage
- `conntrack -L` : list all currently tracked connections
- `conntrack -E` : display a real-time event log of connection changes
- `conntrack -E -o timestamp` : display a real-time event log of connection changes and associated timestamps
- `conntrack -E -s ip_address` : display a real-time event log of connection changes for a specific IP address
- `conntrack -D -s ip_address` : delete all flows for a specific source IP address

## Categories
network, desktop, monitoring

## Links
- More information: https://manned.org/conntrack
- tldr page: https://tldr.inbrowser.app/pages/linux/conntrack

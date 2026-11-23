# ufw reject

Reject traffic through the firewall. Similar to `ufw deny`, but sends a rejection response instead of silently dropping packets.

## Usage
- `sudo ufw reject port` : reject all traffic on a port
- `sudo ufw reject port/protocol` : reject traffic for a protocol on a port
- `sudo ufw reject in protocol comment 'comment'` : reject incoming traffic for a protocol and add a comment for documentation
- `sudo ufw reject from source_address` : reject all traffic from a source address
- `sudo ufw reject from 192.168.13.0/24` : reject all incoming traffic from the subnet 192.168.13.0/24
- `sudo ufw reject from source_address to destination_address port port proto protocol` : reject traffic from a source address to a destination on a specific port and protocol
- `sudo ufw reject in on interface to destination_address` : reject all incoming traffic on an interface to a destination IP

## Categories
network, users, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-reject

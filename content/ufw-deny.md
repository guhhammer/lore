# ufw deny

Block traffic through the firewall.

## Usage
- `sudo ufw deny port` : deny all traffic on a port
- `sudo ufw deny port/protocol` : deny traffic for a protocol on a port
- `sudo ufw deny in protocol comment 'comment'` : deny incoming traffic for a protocol and add a comment for documentation
- `sudo ufw deny from source_address` : deny all traffic from a source address
- `sudo ufw deny from 192.168.13.0/24` : deny all incoming traffic from the subnet 192.168.13.0/24
- `sudo ufw deny from 192.168.1.12 to 192.168.1.100 port 8080 proto udp` : deny UDP traffic from 192.168.1.12 to 192.168.1.100 on port 8080
- `sudo ufw deny in on eth0 to 192.168.1.100 proto igmp` : deny all incoming IGMP traffic to 192.168.1.100 on the eth0 interface

## Categories
network, users, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-deny

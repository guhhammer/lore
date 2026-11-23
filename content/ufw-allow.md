# ufw allow

Allow traffic through the firewall.

## Usage
- `sudo ufw allow port` : allow all traffic on a port
- `sudo ufw allow port/protocol` : allow traffic for a protocol on a port
- `sudo ufw allow in protocol comment 'comment'` : allow incoming traffic for a protocol and add a comment for documentation
- `sudo ufw allow from source_address` : allow all traffic from a source address
- `sudo ufw allow from 192.168.13.0/24` : allow all incoming traffic from the subnet 192.168.13.0/24
- `sudo ufw allow from 192.168.1.12 to 192.168.1.100 port 443 proto tcp` : allow TCP traffic from 192.168.1.12 to 192.168.1.100 on port 443
- `sudo ufw allow in on eth0 to 192.168.1.100 proto gre` : allow all incoming GRE traffic to 192.168.1.100 on the eth0 interface

## Categories
network, users, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-allow

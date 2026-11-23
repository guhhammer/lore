# ufw route

Manage rules for traffic routed or forwarded through the host. IP forwarding must be enabled in the kernel for routed traffic to pass.

## Usage
- `sudo ufw route allow in on eth0 out on eth1` : allow routing traffic from one interface to another
- `sudo ufw route allow from 10.0.0.0/24 to destination_address port port proto protocol` : allow routing traffic from a subnet to a destination on a specific port and protocol
- `sudo ufw route deny to destination_address` : deny routing traffic to a destination address
- `sudo ufw route reject out on interface` : reject routed traffic leaving through an interface
- `sudo ufw route insert number allow|deny|reject|limit from source_address` : insert a route rule for a source address at a specific position
- `sudo ufw route delete rule` : delete a route rule
- `sudo ufw route delete deny to 192.168.0.12` : delete a route rule that denies forwarded traffic to 192.168.0.12

## Categories
network, users, system

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-route

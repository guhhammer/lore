# ufw

Uncomplicated Firewall. Frontend for `iptables` aiming to make configuration of a firewall easier. Some subcommands such as `allow`, `enable`, `delete`, etc. have their own usage documentation.

## Usage
- `sudo ufw enable|disable` : enable/Disable `ufw`
- `sudo ufw status numbered` : show `ufw` rules, along with their numbers
- `sudo ufw allow 5432 comment "Service"` : allow incoming traffic on port 5432 on this host with a comment identifying the service
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : allow only TCP traffic from 192.168.0.4 to any address on this host, on port 22
- `sudo ufw allow in on eth0 from 192.168.0.0/24 to any port 53` : allow traffic from any IP on the 192.168.0/24 subnet on this host, through port 53, only on the eth0 interface
- `sudo ufw deny 80` : deny traffic on port 80 on this host
- `sudo ufw deny proto udp from any to any port 8412:8500` : deny all UDP traffic to ports in range 8412:8500
- `sudo ufw delete rule_number` : delete a particular rule. The rule number can be retrieved from the `ufw status numbered` command

## Categories
network, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw

# iptables

Configure tables, chains, and rules of the Linux kernel IPv4 firewall. Use `ip6tables` to set rules for IPv6 traffic.

## Usage
- `sudo iptables -vnL --line-numbers` : view chains, rules, packet/byte counters, and line numbers for the filter table
- `sudo iptables -P chain rule` : set chain policy rule
- `sudo iptables -A chain -s ip_address -j rule` : append rule to chain policy for IP
- `sudo iptables -A chain -s ip_address -p tcp|udp|icmp|... --dport port -j rule` : append rule to chain policy for IP considering protocol and port
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : add a NAT rule to translate all traffic from the `192.168.0.0/24` subnet to the host's public IP
- `sudo iptables -D chain rule_line_number` : delete chain rule

## See also
- iptables-save
- iptables-restore

## Categories
network, security

## Links
- More information: https://manned.org/iptables
- tldr page: https://tldr.inbrowser.app/pages/linux/iptables

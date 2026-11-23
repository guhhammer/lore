# ip rule

IP routing policy database management.

## Usage
- `ip ru` : display the routing policy
- `sudo ip ru a from all lookup table_id` : create a new generic routing rule with a higher priority than `main`
- `sudo ip ru a from 192.168.178.2/32 lookup table_id` : add a new rule based on packet source addresses
- `sudo ip ru a to 192.168.178.2/32 lookup table_id` : add a new rule based on packet destination addresses
- `sudo ip ru d from 192.168.178.2/32` : delete a rule based on packet source addresses
- `sudo ip ru f` : remove all routing rules
- `ip ru s >path/to/ip_rules.dat` : save all rules to a file
- `sudo ip <path/to/ip_rules.dat ru r` : restore all rules from a file

## See also
- ip route

## Categories
users, network, files

## Links
- More information: https://manned.org/ip-rule
- tldr page: https://tldr.inbrowser.app/pages/linux/ip-rule

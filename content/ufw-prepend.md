# ufw prepend

Add a firewall rule to the top of the ruleset. Unlike `ufw insert`, this does not require an existing rule number.

## Usage
- `sudo ufw --dry-run prepend rule` : simulate prepending a rule without applying it
- `sudo ufw prepend deny from source_address` : deny all traffic from a source address, ahead of all existing rules
- `sudo ufw prepend deny from 192.168.0.0/24` : deny all traffic from a subnet, ahead of all existing rules
- `sudo ufw prepend reject from source_address to any port port` : reject traffic from a source address to a specific port

## Categories
network, users, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-prepend

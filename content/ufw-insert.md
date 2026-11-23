# ufw insert

Insert a firewall rule at a specific position in the ruleset. The position must be an existing rule number, as shown by `ufw status numbered`.

## Usage
- `sudo ufw --dry-run insert number rule` : simulate inserting a rule without applying it
- `sudo ufw insert 1 deny from source_address` : insert a rule denying traffic from a source address at the top of the ruleset
- `sudo ufw insert number allow port/protocol` : insert a rule allowing traffic for a protocol on a port at a specific position
- `sudo ufw insert number reject from 192.168.0.0/24 to any port port` : insert a rule rejecting traffic from a subnet to a port
- `sudo ufw insert number limit 22/tcp` : insert a rate limiting rule at a specific position

## Categories
network, users, security

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-insert

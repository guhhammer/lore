# ufw limit

Rate limit connections to deny an IP initiating 6 or more attempts within 30 seconds.

## Usage
- `sudo ufw --dry-run limit profile` : simulate a rule without applying it
- `sudo ufw limit profile` : rate limit a specific service by profile
- `sudo ufw limit 22/tcp` : rate limit a specific port and protocol
- `sudo ufw limit from 192.168.0.0/16 to any port 22 proto tcp` : rate limit only from a specific source address or subnet
- `sudo ufw limit in on eth0 to any port 22` : rate limit on a specific network interface
- `sudo ufw delete limit service|port/tcp` : delete a limit rule

## Categories
network, users

## Links
- More information: https://manned.org/ufw
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-limit

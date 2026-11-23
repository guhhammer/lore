# pihole

Manage the Pi-hole ad-blocking DNS server.

## Usage
- `pihole status` : check Pi-hole's status
- `sudo pihole -up` : update Pi-hole and Gravity
- `pihole enable|disable` : start or stop the daemon
- `pihole reloaddns` : update the lists and flush the cache without restarting the DNS server
- `pihole -g` : update the list of ad-serving domains
- `pihole allow|deny example.com` : allow or deny the specified domain
- `pihole -q example.com` : search the lists for a domain
- `pihole -t` : open a real-time log of connections

## Categories
network

## Links
- More information: https://docs.pi-hole.net/main/pihole-command/
- tldr page: https://tldr.inbrowser.app/pages/linux/pihole

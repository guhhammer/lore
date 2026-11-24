# the internet does not work

Go step by step from this machine out: its address, the router, the
internet by number, then by name.

## Steps
- `ip -br addr` : the network interfaces and their addresses; one should be UP with an address (Linux)
- `ip route` : the default route, through the router
- `ping -c 3 1.1.1.1` : reach the internet by number
- `ping -c 3 example.com` : reach it by name, which needs DNS
- `resolvectl status` : the DNS servers in use (systemd; else read /etc/resolv.conf)
- `curl -I https://example.com` : a whole web request, with the answer's headers
- `tracepath 1.1.1.1` : where on the way the packets stop

## Notes
- Numbers work but names do not: DNS is the problem.
- No default route, or no address: the problem is the cable, the Wi-Fi or DHCP, before the internet.
- macOS: `ifconfig`, `netstat -rn` for the route, `scutil --dns` for DNS, and `traceroute`.

## See also
- ip
- ping
- curl
- tracepath

## Categories
network

# ping6.py

Check if an IPv6 host is reachable using ICMPv6. Sends ICMPv6 echo requests and listens for echo replies. Note: Requires root privileges for raw socket access (e.g., run with `sudo`). Part of the Impacket suite.

## Usage
- `ping6.py source_ipv6 destination_ipv6` : ping an IPv6 host from a specified source IPv6 address
- `ping6.py 2001:db8::1 2001:db8::2` : ping 2001:db8::2 from 2001:db8::1

## Categories
network, users

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/ping6.py

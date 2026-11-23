# ping.py

Check if an IPv4 host is reachable using ICMP. Sends ICMP echo requests and listens for echo replies. Note: Requires root privileges for raw socket access (e.g., run with `sudo`). Part of the Impacket suite.

## Usage
- `ping.py source_ipv4 destination_ipv4` : ping a host from a specified source IPv4 address
- `ping.py 192.168.1.10 192.168.1.100` : ping 192.168.1.100 from 192.168.1.10

## Categories
network, users

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/ping.py

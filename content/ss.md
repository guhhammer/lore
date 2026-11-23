# ss

Utility to investigate sockets.

## Usage
- `ss -a --tcp|--udp|--raw|--unix` : show all TCP/UDP/RAW/UNIX sockets
- `ss state|exclude bucket|big|connected|synchronized|...` : filter TCP sockets by states, only/exclude
- `ss -t src :443` : show all TCP sockets connected to the local HTTPS port (443)
- `ss -lt src :8080` : show all TCP sockets listening on the local 8080 port
- `ss -pt dst :ssh` : show all TCP sockets along with processes connected to a remote SSH port
- `ss -u 'sport == :source_port and dport == :destination_port'` : show all UDP sockets connected on specific source and destination ports
- `ss -4t src 192.168/16` : show all TCP IPv4 sockets locally connected on the subnet 192.168.0.0/16
- `ss -K dst ip_address dport = port` : kill IPv4 or IPv6 Socket Connection with a specific destination IP and port

## Categories
network

## Links
- More information: https://manned.org/ss
- tldr page: https://tldr.inbrowser.app/pages/linux/ss

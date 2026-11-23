# nping

Network packet generation tool/ping utility.

## Usage
- `nping example.com` : ping a specified host using ICMP if the user is allowed to, otherwise using TCP
- `nping --icmp --privileged example.com` : ping a specified host using ICMP assuming that the user is allowed to do so
- `nping --udp example.com` : ping a specified host using UDP
- `nping --tcp --dest-port 443 example.com` : ping a specified host on a given port using TCP
- `nping --count 10 example.com` : ping a certain number of times
- `nping --delay 5s example.com` : wait a certain amount of time between each ping
- `nping --interface eth0 example.com` : send the request over a specified interface
- `nping 10.0.0.1-10` : ping an IP range

## Categories
network

## Links
- More information: https://nmap.org/nping/
- tldr page: https://tldr.inbrowser.app/pages/common/nping

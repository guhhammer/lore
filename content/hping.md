# hping

This command is an alias of `hping3`.

## Usage
- `hping3 -1 -c 4 ip_or_hostname` : ping a destination with 4 ICMP ping requests
- `hping3 -2 -p 80 -S ip_or_hostname` : ping an IP address over UDP on port 80
- `hping3 -V -S -p 80 -s 5090 ip_or_hostname` : scan TCP port 80, scanning from the specific local source port 5090
- `hping3 -T -V -S -p 80 ip_or_hostname` : traceroute using a TCP scan to a specific destination port
- `hping3 -8 80,3000,9000 -S ip_or_hostname` : scan a set of TCP ports on a specific IP address
- `hping3 -c 2 -V -p 80 -A ip_or_hostname` : perform a TCP ACK scan to check if a given host is alive
- `hping3 --flood -p 80 -S ip_or_hostname` : perform a charge test on port 80

## Categories
network, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/hping
- Wikipedia: https://en.wikipedia.org/wiki/Hping

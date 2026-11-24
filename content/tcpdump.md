# tcpdump

Dump traffic on a network.

## Usage
- `tcpdump -D` : list available network interfaces
- `sudo tcpdump -i eth0` : capture the traffic of a specific interface
- `sudo tcpdump -A tcp` : capture all TCP traffic showing contents (ASCII) in console
- `sudo tcpdump host www.example.com` : capture the traffic from or to a host
- `sudo tcpdump -i eth0 src 192.168.1.1 and dst 192.168.1.2 and dst port 80` : capture the traffic from a specific interface, source, destination, and destination port
- `sudo tcpdump net 192.168.1.0/24` : capture the traffic of a network
- `sudo tcpdump -w path/to/dumpfile.pcap port not 22` : capture all traffic except traffic over port 22 and write to a dump file
- `tcpdump -r path/to/dumpfile.pcap` : read from a given dump file

## Categories
network, users, files

## Links
- More information: https://www.tcpdump.org/manpages/tcpdump.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/tcpdump
- Wikipedia: https://en.wikipedia.org/wiki/Tcpdump

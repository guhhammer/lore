# tcpdump

tcpdump prints the packets that cross a network interface, filtered the way you ask. It shows what really goes over the wire.

## Tasks
### Start listening
- `sudo tcpdump -i any` : watch every interface
- `sudo tcpdump -i NAME` : watch one interface
- `sudo tcpdump -D` : list the interfaces tcpdump can use
- `sudo tcpdump -n -i NAME` : do not turn addresses into names (faster, clearer)
- `sudo tcpdump -nn -i NAME` : no names for ports either
- `sudo tcpdump -c 20 -i NAME` : stop after 20 packets
### Filter
- `sudo tcpdump -i NAME host 10.0.0.5` : traffic to or from one address
- `sudo tcpdump -i NAME src 10.0.0.5` : only what it sends
- `sudo tcpdump -i NAME port 443` : one port
- `sudo tcpdump -i NAME tcp` : only TCP (also udp, icmp, arp)
- `sudo tcpdump -i NAME net 192.168.1.0/24` : a whole network
- `sudo tcpdump -i NAME 'tcp port 80 and host 10.0.0.5'` : combine with and, or, not
- `sudo tcpdump -i NAME 'tcp[tcpflags] & tcp-syn != 0'` : only SYN packets
### Read the content
- `sudo tcpdump -A -i NAME port 80` : print the payload as text
- `sudo tcpdump -X -i NAME port 53` : payload in hex and text
- `sudo tcpdump -v -i NAME` : more detail per packet (-vv, -vvv for more)
- `sudo tcpdump -tttt -i NAME` : full date and time on each line
### Save and replay
- `sudo tcpdump -i NAME -w capture.pcap` : write packets to a file
- `tcpdump -r capture.pcap` : read a saved capture
- `sudo tcpdump -i NAME -s 0 -w capture.pcap port 443` : keep whole packets of one port
- `sudo tcpdump -i NAME -G 3600 -w cap-%H.pcap` : start a new file every hour
- `tcpdump -nn -r capture.pcap | head` : first lines of a capture

## Notes
- It needs root (or the capture capability); capturing other people's traffic is only fine on your own network.
- Put the filter in quotes when it has brackets or parentheses.
- HTTPS content is encrypted: you see the connection, not the page.
- Open a saved `.pcap` in Wireshark for a friendlier view.

## See also
- ss
- ip
- wireshark
- nmap

## Links
- Manual: https://www.tcpdump.org/manpages/tcpdump.1.html
- Filter syntax: https://www.tcpdump.org/manpages/pcap-filter.7.html

## Categories
network monitoring

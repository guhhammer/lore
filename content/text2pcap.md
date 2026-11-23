# text2pcap

Turns a hex dump of packets into a capture file Wireshark can read.

## Usage
- `text2pcap dump.txt out.pcap` : convert a hex dump to a capture file
- `text2pcap -e 0x800 dump.txt out.pcap` : add an Ethernet header with a given type
- `text2pcap -u 1000,2000 dump.txt out.pcap` : wrap the data in a UDP header

## Categories
files

## Links
- Documentation: https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml

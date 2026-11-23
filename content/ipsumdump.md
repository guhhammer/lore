# ipsumdump

Summarize TCP/IP dumps into a human and machine readable ASCII format.

## Usage
- `ipsumdump --src --dst path/to/file.pcap` : print the source and destination IP addresses of all packets in a PCAP file
- `ipsumdump --interface eth0 -tsSdDp` : print the timestamps, source address, source port, destination address, destination port, and protocol of all packets read from a given network interface
- `ipsumdump --src --dst --length --anonymize path/to/file.pcap` : print the anonymized source address, anonymized destination address, and IP packet length of all packets in a PCAP file

## Categories
network, files, documents

## Links
- More information: https://manned.org/ipsumdump
- tldr page: https://tldr.inbrowser.app/pages/common/ipsumdump

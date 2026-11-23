# sngrep

Display SIP calls message flows from terminal.

## Usage
- `sngrep -I path/to/file.pcap` : visualize SIP packets from a PCAP file
- `sngrep -crI path/to/file.pcap` : visualize only dialogs starting with INVITE packets with RTP packets from a PCAP file
- `sngrep -cr` : display a real-time interface with only dialogs starting with INVITE packets, including RTP packets
- `sngrep -NO path/to/file.pcap` : only capture packets without interface to a file

## Categories
files, desktop, network

## Links
- More information: https://manned.org/sngrep
- tldr page: https://tldr.inbrowser.app/pages/common/sngrep

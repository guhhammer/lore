# wireshark

A graphical network protocol analyzer: captures packets from an interface or opens a capture file and lets you inspect them.

## Usage
- `wireshark` : open the program
- `wireshark capture.pcap` : open a saved capture file
- `wireshark -i eth0 -k` : start capturing on an interface at once
- `wireshark -Y 'http' capture.pcap` : open a file with a display filter applied

## Notes
Capturing live traffic usually needs extra permission. Only capture on networks you are allowed to monitor.

## Categories
network, files

## Links
- Documentation: https://www.wireshark.org/docs/wsug_html_chunked/
- Wikipedia: https://en.wikipedia.org/wiki/Wireshark

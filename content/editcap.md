# editcap

Edits or converts packet capture files: cut out packets, change the format, split a big capture.

## Usage
- `editcap -r in.pcap out.pcap 1-100` : keep only packets 1 to 100
- `editcap -d in.pcap out.pcap` : remove duplicate packets
- `editcap -F pcap in.pcapng out.pcap` : convert to another capture format
- `editcap -c 10000 in.pcap part.pcap` : split into files of 10000 packets each

## Categories
network

## Links
- Documentation: https://www.wireshark.org/docs/man-pages/wireshark.html

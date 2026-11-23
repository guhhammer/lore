# mergecap

Combines several packet capture files into one.

## Usage
- `mergecap -w out.pcap a.pcap b.pcap` : merge files in time order into out.pcap
- `mergecap -a -w out.pcap a.pcap b.pcap` : append the files one after another instead of by time
- `mergecap -F pcapng -w out.pcapng a.pcap b.pcap` : choose the output format

## Categories
network

## Links
- Documentation: https://www.wireshark.org/docs/man-pages/wireshark.html

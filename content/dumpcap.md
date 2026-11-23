# dumpcap

A network traffic dump tool.

## Usage
- `dumpcap -D` : display available interfaces
- `dumpcap -i 1` : capture packets on a specific interface
- `dumpcap -i 1 -w path/to/output_file.pcapng` : capture packets to a specific location
- `dumpcap -i 1 -w path/to/output_file.pcapng -b filesize:500000 -b files:10` : write to a ring buffer with a specific max file limit of a specific size

## Categories
network, files

## Links
- More information: https://www.wireshark.org/docs/man-pages/dumpcap.html
- tldr page: https://tldr.inbrowser.app/pages/common/dumpcap

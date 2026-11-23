# pcapfix

Repair damaged or corrupted PCAP and PcapNG files.

## Usage
- `pcapfix path/to/file.pcapng` : repair a PCAP/PCapNG file (Note: For PCAP files, only the first 262144 bytes of each packet are scanned)
- `pcapfix --deep-scan path/to/file.pcap` : repair an entire PCAP file
- `pcapfix --outfile path/to/repaired.pcap path/to/file.pcap` : repair a PCAP/PcapNG file and write the repaired file to the specified location
- `pcapfix --pcapng path/to/file.pcapng` : treat the specified file as a PcapNG file, ignoring automatic recognition
- `pcapfix --verbose path/to/file.pcap` : repair a file and show the process in detail

## Categories
files, network

## Links
- More information: https://f00l.de/pcapfix/
- tldr page: https://tldr.inbrowser.app/pages/common/pcapfix

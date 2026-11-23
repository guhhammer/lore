# ipaggcreate

Produce aggregate statistics of TCP/IP dumps.

## Usage
- `ipaggcreate --src path/to/file.pcap` : count the number of packets sent from each source address appearing in a PCAP file
- `ipaggcreate --interface eth0 --length` : group and count packets read from a network interface by IP packet length
- `ipaggcreate --address-pairs --bytes path/to/file.pcap` : count the number of bytes sent between each address pair appearing in a PCAP file

## Categories
files, network, monitoring

## Links
- More information: https://manned.org/ipaggcreate
- tldr page: https://tldr.inbrowser.app/pages/common/ipaggcreate

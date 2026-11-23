# wol

Client for sending Wake-on-LAN magic packets.

## Usage
- `wol mac_address` : send a WoL packet to a device
- `wol -i ip_address mac_address` : send a WoL packet to a device in another subnet based on its IP
- `wol -h hostname mac_address` : send a WoL packet to a device in another subnet based on its hostname
- `wol -p port_number mac_address` : send a WoL packet to a specific port on a host
- `wol -f path/to/file` : read hardware addresses, IP addresses/hostnames, optional ports, and SecureON passwords from a file
- `wol -v mac_address` : turn on verbose output

## Categories
network, files, hardware

## Links
- More information: https://sourceforge.net/projects/wake-on-lan/
- tldr page: https://tldr.inbrowser.app/pages/linux/wol

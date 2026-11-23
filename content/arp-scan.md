# arp-scan

Send ARP packets to hosts (specified as IP addresses or hostnames) to scan the local network.

## Usage
- `arp-scan -l` : scan the current local network
- `arp-scan 10.0.0.1` : scan a specific host
- `arp-scan 192.168.1.1/24` : scan an IP network with a custom bitmask
- `arp-scan 127.0.0.0-127.0.0.31` : scan an IP network within a custom range
- `arp-scan 10.0.0.0:255.255.255.0` : scan an IP network with a custom net mask
- `arp-scan 10.0.0.1 -I interface_name` : specify an interface to use for scanning

## See also
- ip neighbor

## Categories
network

## Links
- More information: https://github.com/royhills/arp-scan
- tldr page: https://tldr.inbrowser.app/pages/common/arp-scan

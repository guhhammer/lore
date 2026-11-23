# dnsmasq

Lightweight DNS, DHCP, TFTP, and PXE server.

## Usage
- `dnsmasq` : start dnsmasq with default configuration
- `dnsmasq --no-daemon` : run dnsmasq in the foreground (for debugging)
- `dnsmasq --conf-file=path/to/config.conf` : specify a custom configuration file
- `dnsmasq --log-queries --log-facility=-` : enable verbose logging
- `dnsmasq --dhcp-range=192.168.0.50,192.168.0.150,12h` : set a DHCP range and lease time
- `dnsmasq --version` : display version

## Categories
network, files

## Links
- More information: https://manned.org/dnsmasq
- tldr page: https://tldr.inbrowser.app/pages/common/dnsmasq

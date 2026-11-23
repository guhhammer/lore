# pfctl

Control the packet filter device.

## Usage
- `sudo pfctl -e` : enable the packet filter
- `sudo pfctl -d` : disable the packet filter
- `sudo pfctl -f path/to/pf.conf` : load rules from a configuration file
- `pfctl -sr` : show all active rules
- `pfctl -s info` : show packet filter status information
- `sudo pfctl -F rules` : flush all rules

## Notes
A FreeBSD command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, users, hardware, bsd

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=pfctl
- tldr page: https://tldr.inbrowser.app/pages/freebsd/pfctl

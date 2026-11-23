# ipcalc

Calculate IP information (subnet, broadcast, host range) from an IP address and netmask.

## Usage
- `ipcalc 192.168.0.1` : display network info for an IP address
- `ipcalc 192.168.0.1/24` : display network info using CIDR notation
- `ipcalc 192.168.0.1 255.255.255.0` : display network info using a dotted decimal netmask
- `ipcalc -b 192.168.0.1` : suppress bitwise output
- `ipcalc -s size1 size2 size3 ... 192.168.0.1` : split a network into specified sized blocks
- `ipcalc -v` : display version

## Categories
network, desktop

## Links
- More information: https://manned.org/ipcalc
- tldr page: https://tldr.inbrowser.app/pages/common/ipcalc

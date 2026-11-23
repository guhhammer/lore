# iw

Show and manipulate wireless devices.

## Usage
- `iw dev wlanX scan` : scan for available wireless networks
- `iw dev wlanX connect ssid` : join an open wireless network
- `iw dev wlanX disconnect` : close the current connection
- `iw dev wlanX link` : show information about the current connection
- `iw dev` : list all physical and logical wireless network interfaces
- `iw phy` : list all wireless capabilities for all physical hardware interfaces
- `iw reg get` : list the kernel's current wireless regulatory domain information
- `iw help` : display help

## See also
- nmcli
- iwctl

## Categories
network, hardware

## Links
- More information: https://wireless.docs.kernel.org/en/latest/en/users/documentation/iw.html
- tldr page: https://tldr.inbrowser.app/pages/linux/iw

# wakeonlan

Send packets to wake-on-LAN (WOL) enabled PCs.

## Usage
- `wakeonlan 01:02:03:04:05:06` : send packets to all devices on the local network (255.255.255.255) by specifying a MAC address
- `wakeonlan 01:02:03:04:05:06 -i 192.168.178.2` : send packet to a specific device via IP address
- `wakeonlan -n 01:02:03:04:05:06` : simulate execution and print the commands without running them
- `wakeonlan -q 01:02:03:04:05:06` : run in quiet mode

## Categories
network

## Links
- More information: https://manned.org/wakeonlan
- tldr page: https://tldr.inbrowser.app/pages/common/wakeonlan

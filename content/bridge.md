# bridge

Show and manipulate network bridge addresses and devices.

## Usage
- `bridge l` : list all bridges and their interfaces
- `bridge v` : show port vlan information
- `sudo bridge v a dev lanX vid vlan_id pvid tagged|untagged` : assign a VLAN to a port
- `sudo bridge v d dev lanX vid vlan_id` : remove a VLAN from a port
- `bridge mo` : watch for changes in bridge interfaces
- `bridge h` : display help

## Categories
network, hardware

## Links
- More information: https://manned.org/bridge
- tldr page: https://tldr.inbrowser.app/pages/linux/bridge

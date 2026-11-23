# brctl

Ethernet bridge administration.

## Usage
- `sudo brctl show` : show a list with information about currently existing Ethernet bridges
- `sudo brctl add bridge_name` : create a new Ethernet bridge interface
- `sudo brctl del bridge_name` : delete an existing Ethernet bridge interface
- `sudo brctl addif bridge_name interface_name` : add an interface to an existing bridge
- `sudo brctl delif bridge_name interface_name` : remove an interface from an existing bridge

## Categories
network, users

## Links
- More information: https://manned.org/brctl
- tldr page: https://tldr.inbrowser.app/pages/linux/brctl

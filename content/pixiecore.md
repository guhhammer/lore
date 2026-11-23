# pixiecore

Manage the network booting of machines.

## Usage
- `pixiecore quick xyz --dhcp-no-bind` : start a PXE boot server which provides a `netboot.xyz` boot image
- `pixiecore quick ubuntu --dhcp-no-bind` : start a new PXE boot server which provides an Ubuntu boot image
- `pixiecore quick --help` : list all available boot images for quick mode

## Categories
network, system, containers

## Links
- More information: https://github.com/danderson/netboot/tree/main/pixiecore
- tldr page: https://tldr.inbrowser.app/pages/common/pixiecore

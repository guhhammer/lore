# lspci

List all PCI devices.

## Usage
- `lspci` : show a brief list of devices
- `lspci -v` : display verbose information (Note: the `-v` flag can be repeated to increase verbosity)
- `lspci -k` : display kernel drivers and modules handling each device
- `lspci -s 00:18.3` : select a specific device
- `lspci -vm` : dump info in a (machine) readable form
- `lspci -nn` : show PCI vendor and device codes as both numbers and names

## Categories
hardware

## Links
- More information: https://manned.org/lspci
- tldr page: https://tldr.inbrowser.app/pages/linux/lspci

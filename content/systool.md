# systool

View system device information by bus, and classes. This command is part of the `sysfs` package.

## Usage
- `systool -b bus -v` : list all attributes of devices of a bus (eg. `pci`, `usb`). View all buses using `ls /sys/bus`
- `systool -c class -v` : list all attributes of a class of devices (eg. `drm`, `block`). View all classes using `ls /sys/class`
- `systool -b bus -D` : show only device drivers of a bus (eg. `pci`, `usb`)

## Categories
hardware, packages

## Links
- More information: https://manned.org/systool
- tldr page: https://tldr.inbrowser.app/pages/linux/systool

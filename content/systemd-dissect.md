# systemd-dissect

Introspect and interact with filesystem OS disk images, specifically Discoverable Disk Images (DDIs).

## Usage
- `systemd-dissect path/to/image.raw` : show general image information about the OS image
- `systemd-dissect -m path/to/image.raw /mnt/image` : mount an OS image
- `systemd-dissect -u /mnt/image` : unmount an OS image
- `systemd-dissect -l path/to/image.raw` : list files in an image
- `systemd-dissect --attach path/to/image.raw` : attach an OS image to an automatically allocated loopback block device and print its path
- `systemd-dissect --detach path/to/device` : detach an OS image from a loopback block device

## Categories
containers, media, disks

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-dissect.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-dissect

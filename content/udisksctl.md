# udisksctl

Interact with `udisksd` to query and manipulate storage devices.

## Usage
- `udisksctl status` : show high-level information about disk drives and block devices
- `udisksctl info -b /dev/sdX` : show detailed information about a device
- `udisksctl info -b /dev/sdXN` : show detailed information about a device partition
- `udisksctl mount -b /dev/sdXN` : mount a device partition and print the mount point
- `udisksctl unmount -b /dev/sdXN` : unmount a device partition
- `udisksctl power-off -b /dev/sdX` : power off a device to safely remove it
- `udisksctl monitor` : monitor the daemon for events

## See also
- mount

## Categories
hardware, disks, databases

## Links
- More information: https://storaged.org/doc/udisks2-api/latest/udisksctl.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/udisksctl

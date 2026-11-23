# vgcreate

Create volume groups combining multiple mass-storage devices.

## Usage
- `sudo vgcreate volume_group /dev/sdXY` : create a new volume group using the specified device
- `sudo vgcreate volume_group /dev/sdXY /dev/sdXZ ...` : create a new volume group using multiple devices
- `sudo vgcreate -s 8M volume_group /dev/sdXY /dev/sdXZ ...` : create a new volume group with custom physical extent size

## See also
- lvm

## Categories
users, disks, hardware

## Links
- More information: https://manned.org/vgcreate
- tldr page: https://tldr.inbrowser.app/pages/linux/vgcreate

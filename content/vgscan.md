# vgscan

Scan for volume groups on all supported Logical Volume Manager (LVM) block devices.

## Usage
- `sudo vgscan` : scan for volume groups and print information about each group found
- `sudo vgscan --mknodes` : scan for volume groups and add the special files in `/dev`, if they don't already exist, needed to access the logical volumes in the found groups

## See also
- lvm
- vgchange

## Categories
disks, users, hardware

## Links
- More information: https://manned.org/vgscan
- tldr page: https://tldr.inbrowser.app/pages/linux/vgscan

# partx

Parse a partition table and tell the kernel about it.

## Usage
- `sudo partx -l path/to/device_or_disk_image` : list the partitions on a block device or disk image
- `sudo partx -a -v path/to/device_or_disk_image` : add all the partitions found in a given block device to the kernel
- `sudo partx -d path/to/device_or_disk_image` : delete all the partitions found from the kernel (does not alter partitions on disk)

## Categories
disks, system, databases

## Links
- More information: https://manned.org/partx
- tldr page: https://tldr.inbrowser.app/pages/linux/partx

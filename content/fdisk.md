# fdisk

Manage partition tables and partitions on a storage drive.

## Usage
- `sudo fdisk -l` : list partitions
- `sudo fdisk /dev/sdX` : start the interactive partition manipulator
- `<m>` : open a help menu
- `<p>` : view the partition table
- `<n>` : create a new partition
- `<d>` : select a partition to delete
- `<w>` : write the changes made
- `<q>` : discard the changes made and quit

## See also
- partprobe
- parted
- cfdisk

## Categories
danger, disks

## Links
- More information: https://manned.org/fdisk
- tldr page: https://tldr.inbrowser.app/pages/linux/fdisk
- Wikipedia: https://en.wikipedia.org/wiki/Fdisk

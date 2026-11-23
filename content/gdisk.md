# gdisk

GPT (GUID Partition Table) disk partitioning tool.

## Usage
- `sudo gdisk -l /dev/sdX` : list partitions
- `sudo gdisk /dev/sdX` : start the interactive partition manipulator
- `<?>` : open a help menu
- `<p>` : print the partition table
- `<n>` : add a new partition
- `<d>` : select a partition to delete
- `<w>` : write table to disk and exit
- `<q>` : quit without saving changes

## See also
- cfdisk
- fdisk
- parted

## Categories
danger, disks, databases

## Links
- More information: https://manned.org/gdisk
- tldr page: https://tldr.inbrowser.app/pages/linux/gdisk

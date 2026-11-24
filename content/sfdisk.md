# sfdisk

Display or manipulate a disk partition table.

## Usage
- `sudo sfdisk -d /dev/sdX >path/to/file.dump` : back up the partition layout to a file
- `sudo sfdisk <path/to/file.dump /dev/sdX` : restore a partition layout
- `sudo sfdisk --part-type /dev/sdX partition_number swap` : set the type of a partition
- `sudo sfdisk --delete /dev/sdX partition_number` : delete a partition
- `sfdisk -h` : display help

## Categories
danger, disks, files, desktop

## Links
- More information: https://manned.org/sfdisk
- tldr page: https://tldr.inbrowser.app/pages/linux/sfdisk
- Wikipedia: https://en.wikipedia.org/wiki/Sfdisk

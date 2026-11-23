# mkfs

Build a Linux filesystem on a hard disk partition. Note: This command is deprecated, use filesystem specific commands like `mkfs.ext4` instead.

## Usage
- `sudo mkfs /dev/sdXY` : build a Linux ext2 filesystem on a partition
- `sudo mkfs -t ext4 /dev/sdXY` : build a filesystem of a specified type
- `sudo mkfs -c -t ntfs /dev/sdXY` : build a filesystem of a specified type and check for bad blocks

## Categories
danger, disks

## Links
- More information: https://manned.org/mkfs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs

# xmount

Convert on-the-fly between multiple input and output hard disk image types with optional write cache support. Create a virtual filesystem using FUSE (Filesystem in Userspace) that contains a virtual representation of the input image.

## Usage
- `xmount --in raw path/to/image.dd --out dmg path/to/mount_point` : mount a `.raw` image file into a DMG container file
- `xmount --cache path/to/cache.ovl --in ewf path/to/image.E?? --out vhd path/to/mount_point` : mount an EWF image file with write-cache support into a VHD file to boot from
- `xmount --offset 2048 --in raw path/to/image.dd --out raw path/to/mount_point` : mount the first partition at sector 2048 into a new `.raw` image file

## Categories
containers, disks, files

## Links
- More information: https://manned.org/xmount
- tldr page: https://tldr.inbrowser.app/pages/linux/xmount

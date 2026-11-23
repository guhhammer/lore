# resize2fs

Resize an ext2, ext3, or ext4 filesystem. Does not resize the underlying partition. The filesystem may have to be unmounted first, read the man page for more details.

## Usage
- `resize2fs /dev/sdXN` : automatically resize a filesystem to its maximum possible size
- `resize2fs -p /dev/sdXN 40G` : resize the filesystem to a size of 40G, displaying a progress bar
- `resize2fs -M /dev/sdXN` : shrink the filesystem to its minimum possible size

## Categories
disks, documents

## Links
- More information: https://manned.org/resize2fs
- tldr page: https://tldr.inbrowser.app/pages/linux/resize2fs

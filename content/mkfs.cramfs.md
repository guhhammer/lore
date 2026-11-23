# mkfs.cramfs

Create a ROM filesystem inside a partition.

## Usage
- `sudo mkfs.cramfs path/to/directory /dev/sdXY` : create a ROM filesystem out of a directory inside partition Y on device X
- `sudo mkfs.cramfs -n volume_name path/to/directory /dev/sdXY` : create a ROM filesystem with a volume-name

## Categories
danger, disks, files

## Links
- More information: https://manned.org/mkfs.cramfs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.cramfs

# mkfs.ntfs

Create an NTFS filesystem inside a partition.

## Usage
- `sudo mkfs.ntfs /dev/sdXY` : create an NTFS filesystem inside partition Y on device X
- `sudo mkfs.ntfs -L volume_label /dev/sdXY` : create filesystem with a volume-label
- `sudo mkfs.ntfs -U uuid /dev/sdXY` : create filesystem with specific UUID

## Categories
disks, users

## Links
- More information: https://manned.org/mkfs.ntfs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.ntfs

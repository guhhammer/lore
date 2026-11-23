# mkfs.btrfs

Create a BTRFS filesystem. Defaults to `raid1`, which specifies 2 copies of a data block spread across 2 different devices.

## Usage
- `sudo mkfs.btrfs /dev/sdXY` : create a Btrfs filesystem on an empty partition
- `sudo mkfs.btrfs -m single -d single /dev/sdX` : create a btrfs filesystem on a single device
- `sudo mkfs.btrfs -m raid1 -d raid1 /dev/sdX /dev/sdY /dev/sdZ ...` : create a btrfs filesystem on multiple devices with raid1
- `sudo mkfs.btrfs -L "label" /dev/sdX /dev/sdY ...` : set a label for the filesystem
- `sudo mkfs.btrfs -f /dev/sdX` : overwrite existing filesystem if one is detected

## Categories
danger, disks, hardware, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.btrfs

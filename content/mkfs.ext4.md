# mkfs.ext4

Create an ext4 filesystem inside a partition.

## Usage
- `sudo mkfs.ext4 /dev/sdXY` : create an ext4 filesystem inside partition Y on device X
- `sudo mkfs.ext4 -L volume_label /dev/sdXY` : create an ext4 filesystem with a volume-label
- `sudo mkfs.ext4 -E root_owner=user_id:group_id /dev/sdXY` : create an ext4 filesystem owned by a specific user and group

## Categories
disks, users

## Links
- More information: https://manned.org/mkfs.ext4
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.ext4

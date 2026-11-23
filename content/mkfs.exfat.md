# mkfs.exfat

Create an exfat filesystem inside a partition.

## Usage
- `sudo mkfs.exfat /dev/sdXY` : create an exfat filesystem inside partition Y on device X
- `sudo mkfs.exfat -L volume_name /dev/sdXY` : create filesystem with a volume-name
- `sudo mkfs.exfat -U volume_id /dev/sdXY` : create filesystem with a volume-id

## Categories
disks, users

## Links
- More information: https://manned.org/mkfs.exfat
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.exfat

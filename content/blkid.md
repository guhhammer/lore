# blkid

List all recognized partitions and their Universally Unique Identifier (UUID).

## Usage
- `sudo blkid` : list all partitions
- `sudo blkid -o list` : list all partitions in a table, including current mountpoints
- `sudo blkid -s UUID -o value /dev/sdXY` : get the UUID of the filesystem on a partition

## Categories
users

## Links
- More information: https://manned.org/blkid
- tldr page: https://tldr.inbrowser.app/pages/linux/blkid

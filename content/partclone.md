# partclone

Copy and restore partitions to and from an image while disregarding empty blocks.

## Usage
- `sudo partclone.ext4|btrfs|fat32|xfs|... -c -s /dev/sdXY -o path/to/backup.img` : copy a partition into an image
- `sudo partclone.ext4|btrfs|fat32|xfs|... -c -s path/to/backup.img -o /dev/sdXY` : restore a partition from an image
- `partclone.ext4|btrfs|fat32|xfs|... -h` : display help

## Categories
files, containers, media

## Links
- More information: https://manned.org/partclone
- tldr page: https://tldr.inbrowser.app/pages/linux/partclone
- Wikipedia: https://en.wikipedia.org/wiki/Partclone

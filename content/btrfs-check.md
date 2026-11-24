# btrfs check

Check or repair a btrfs filesystem.

## Usage
- `sudo btrfs c path/to/partition` : check a btrfs filesystem
- `sudo btrfs c --repair path/to/partition` : check and repair a btrfs filesystem (dangerous)
- `sudo btrfs c -p path/to/partition` : show the progress of the check
- `sudo btrfs c --check-data-csum path/to/partition` : verify the checksum of each data block (if the filesystem is good)
- `sudo btrfs c -s n path/to/partition` : use the `n`-th superblock (`n` can be 0, 1, or 2)
- `sudo btrfs c --repair --init-csum-tree path/to/partition` : rebuild the checksum tree
- `sudo btrfs c --repair --init-extent-tree path/to/partition` : rebuild the extent tree

## Categories
disks, files, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-check.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-check
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

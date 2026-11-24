# btrfs inspect-internal

Query internal information of a btrfs filesystem.

## Usage
- `sudo btrfs i dump-s path/to/partition` : print superblock's information
- `sudo btrfs i dump-s -a path/to/partition` : print superblock's and all of its copies' information
- `sudo btrfs i dump-t path/to/partition` : print filesystem's metadata information
- `sudo btrfs i i n path/to/btrfs_mount` : print list of files in inode `n`-th
- `sudo btrfs i lo logical_address path/to/btrfs_mount` : print list of files at a given logical address
- `sudo btrfs i t path/to/partition` : print stats of root, extent, csum, and fs trees

## Categories
disks, documents, files

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-inspect-internal
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

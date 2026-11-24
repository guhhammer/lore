# btrfs property

Get, set, or list properties for a BTRFS filesystem object (files, directories, subvolumes, filesystems, or devices).

## Usage
- `sudo btrfs p l path/to/btrfs_object` : list available properties (and descriptions) for the given btrfs object
- `sudo btrfs p g path/to/btrfs_object` : get all properties for the given btrfs object
- `sudo btrfs p g path/to/btrfs_filesystem label` : get the `label` property for the given btrfs filesystem or device
- `sudo btrfs p g -t subvol|filesystem|inode|device path/to/btrfs_filesystem` : get all object type-specific properties for the given btrfs filesystem or device
- `sudo btrfs p s path/to/btrfs_inode compression zstd|zlib|lzo|none` : set the `compression` property for a given btrfs inode (either a file or directory)
- `sudo btrfs p s path/to/btrfs_subvolume ro true|false` : set the `ro` (read-only) property for a given btrfs subvolume

## Categories
files, disks, hardware

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-property.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-property
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

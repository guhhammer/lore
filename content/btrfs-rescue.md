# btrfs rescue

Try to recover a damaged btrfs filesystem.

## Usage
- `sudo btrfs resc ch path/to/partition` : rebuild the filesystem metadata tree (very slow)
- `sudo btrfs resc fix-de path/to/partition` : fix device size alignment related problems (e.g. unable to mount the filesystem with super total bytes mismatch)
- `sudo btrfs resc s path/to/partition` : recover a corrupted superblock from correct copies (recover the root of filesystem tree)
- `sudo btrfs resc z path/to/partition` : recover from an interrupted transactions (fixes log replay problems)
- `sudo btrfs resc c` : create a `/dev/btrfs-control` control device when `mknod` is not installed

## Categories
disks, users, files

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-rescue.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-rescue
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

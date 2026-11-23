# btrfs filesystem

Manage btrfs filesystems.

## Usage
- `btrfs f u path/to/btrfs_mount` : show filesystem usage (optionally run as root to show detailed information)
- `sudo btrfs f sh path/to/btrfs_mount` : show usage by individual devices
- `sudo btrfs f de -v path/to/file` : defragment a single file on a btrfs filesystem (avoid while a deduplication agent is running)
- `sudo btrfs f de -v -r path/to/directory` : defragment a directory recursively (does not cross subvolume boundaries)
- `sudo btrfs f sy path/to/btrfs_mount` : force syncing unwritten data blocks to disk(s)
- `sudo btrfs f du -s path/to/directory` : summarize disk usage for the files in a directory recursively
- `sudo btrfs f m --size 8g --uuid clear|random|time|uuid_value path/to/swapfile` : create a swap file

## Categories
files, disks, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-filesystem.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-filesystem

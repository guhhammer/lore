# btrfs scrub

Scrub btrfs filesystems to verify data integrity. It is recommended to run a scrub once a month.

## Usage
- `sudo btrfs sc start path/to/btrfs_mount` : start a scrub
- `sudo btrfs sc status path/to/btrfs_mount` : show the status of an ongoing or last completed scrub
- `sudo btrfs sc c path/to/btrfs_mount` : cancel an ongoing scrub
- `sudo btrfs sc r path/to/btrfs_mount` : resume a previously cancelled scrub
- `sudo btrfs sc start -B path/to/btrfs_mount` : start a scrub, but do not put the program in the Background
- `sudo btrfs sc start -q path/to/btrfs_mount` : start a scrub in quiet mode (does not print errors or statistics)

## Categories
files, users

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-scrub
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

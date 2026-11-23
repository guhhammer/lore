# findmnt

Find your filesystem.

## Usage
- `findmnt` : list all mounted filesystems
- `findmnt /dev/sdb1` : search for a device
- `findmnt /` : search for a mountpoint
- `findmnt -t ext4,virtiofs,btrfs,...` : find filesystems in specific type
- `findmnt LABEL=BigStorage` : find filesystems with specific label
- `findmnt -x --verbose` : check mount table content in detail and verify `/etc/fstab`

## Categories
disks

## Links
- More information: https://manned.org/findmnt
- tldr page: https://tldr.inbrowser.app/pages/linux/findmnt

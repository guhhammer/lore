# xfs_admin

Tune an XFS filesystem.

## Usage
- `sudo xfs_admin -l /dev/sdX` : display the filesystem label
- `sudo xfs_admin -L "label" /dev/sdX` : set the filesystem label
- `sudo xfs_admin -u /dev/sdX` : display the filesystem UUID
- `sudo xfs_admin -U uuid /dev/sdX` : set the filesystem UUID (use with caution)
- `sudo xfs_admin -U generate /dev/sdX` : generate a new UUID for the filesystem
- `xfs_admin` : display help

## Categories
disks, users, desktop

## Links
- More information: https://manned.org/xfs_admin
- tldr page: https://tldr.inbrowser.app/pages/linux/xfs_admin

# zfs

Manage ZFS filesystems.

## Usage
- `zfs list` : list all available zfs filesystems
- `zfs create pool_name/filesystem_name` : create a new ZFS filesystem
- `zfs destroy pool_name/filesystem_name` : delete a ZFS filesystem
- `zfs snapshot pool_name/filesystem_name@snapshot_name` : create a Snapshot of a ZFS filesystem
- `zfs set compression=on pool_name/filesystem_name` : enable compression on a filesystem
- `zfs set mountpoint=/path/to/mount_point pool_name/filesystem_name` : change mountpoint for a filesystem

## Categories
disks

## Links
- More information: https://manned.org/zfs
- tldr page: https://tldr.inbrowser.app/pages/common/zfs

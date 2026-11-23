# umount

Unlink a filesystem from its mount point, making it no longer accessible. A filesystem cannot be unmounted when it is busy.

## Usage
- `umount path/to/device_file` : unmount a filesystem, by passing the path to the source it is mounted from
- `umount path/to/mounted_directory` : unmount a filesystem, by passing the path to the target where it is mounted
- `umount -a` : unmount all mounted filesystems (except the `proc` filesystem)

## Categories
disks

## Links
- More information: https://man.openbsd.org/umount
- tldr page: https://tldr.inbrowser.app/pages/common/umount

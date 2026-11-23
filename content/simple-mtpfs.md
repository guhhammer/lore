# simple-mtpfs

Mount MTP devices (like Android phones) as a filesystem using FUSE.

## Usage
- `simple-mtpfs -l` : list available MTP devices
- `simple-mtpfs path/to/mount_point` : mount a device to a directory
- `simple-mtpfs --device number path/to/mount_point` : mount a specific device to a directory (useful when multiple devices are connected)
- `umount path/to/mount_point` : unmount the filesystem

## Categories
disks, hardware, files

## Links
- More information: https://manned.org/simple-mtpfs
- tldr page: https://tldr.inbrowser.app/pages/common/simple-mtpfs

# diskutil

Utility to manage local disks and volumes. Some subcommands such as `partitiondisk` have their own usage documentation.

## Usage
- `diskutil list` : list all currently available disks, partitions, and mounted volumes
- `diskutil repairVolume /dev/disk` : repair the filesystem data structures of a volume
- `diskutil unmountDisk /dev/disk` : unmount a volume
- `diskutil eject /dev/diskX` : eject a CD/DVD (unmount first)

## Categories
disks, documents, monitoring, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/diskutil

# btrfs device

Manage devices in a btrfs filesystem.

## Usage
- `sudo btrfs d a path/to/block_device1 path/to/block_device2 ... path/to/btrfs_filesystem` : add one or more devices to a btrfs filesystem
- `sudo btrfs d rem path/to/device1|device_id1 path/to/device2|device_id2 ...` : remove a device from a btrfs filesystem
- `sudo btrfs d st path/to/btrfs_filesystem` : display error statistics
- `sudo btrfs d sc -d` : scan all disks and inform the kernel of all detected btrfs filesystems
- `sudo btrfs d u path/to/btrfs_filesystem` : display detailed per-disk allocation statistics

## Categories
hardware, disks, files

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-device.html
- tldr page: https://tldr.inbrowser.app/pages/linux/btrfs-device
- Wikipedia: https://en.wikipedia.org/wiki/Btrfs

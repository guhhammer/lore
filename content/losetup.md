# losetup

Set up and control loop devices.

## Usage
- `losetup -a` : list loop devices with detailed info
- `sudo losetup /dev/loop /path/to/file` : attach a file to a given loop device
- `sudo losetup --show -P -f /path/to/file` : attach a file to a new free loop device and scan the device for partitions
- `sudo losetup -r /dev/loop /path/to/file` : attach a file to a read-only loop device
- `sudo losetup -D` : detach all loop devices
- `sudo losetup -d /dev/loop` : detach a given loop device

## Categories
hardware, files, users

## Links
- More information: https://manned.org/losetup
- tldr page: https://tldr.inbrowser.app/pages/linux/losetup

# bcachefs device

Manage devices within a running `bcachefs` filesystem.

## Usage
- `sudo bcachefs device add --label group.name path/to/mount_point path/to/device` : format and add a new device to an existing filesystem
- `bcachefs device evacuate path/to/device` : migrate data off a device to prepare for removal
- `bcachefs device remove path/to/device` : permanently remove a device from a filesystem

## Categories
hardware, disks, files

## Links
- More information: https://bcachefs-docs.readthedocs.io/en/latest/mgmt-devicemanagement.html
- tldr page: https://tldr.inbrowser.app/pages/linux/bcachefs-device
- Wikipedia: https://en.wikipedia.org/wiki/Bcachefs

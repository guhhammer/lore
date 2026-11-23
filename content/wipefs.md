# wipefs

Wipe filesystem, raid, or partition-table signatures from a device.

## Usage
- `sudo wipefs /dev/sdX` : display signatures for specified device
- `sudo wipefs -a /dev/sdX` : wipe all available signature types for a specific device with no recursion into partitions
- `sudo wipefs -a /dev/sdX*` : wipe all available signature types for the device and partitions using a `glob` pattern
- `sudo wipefs -a -n /dev/sdX` : simulate wiping all signature types from a device without actually modifying it
- `sudo wipefs -a -f /dev/sdX` : force wipe, even if the filesystem is mounted

## Categories
danger, disks, hardware, users

## Links
- More information: https://manned.org/wipefs
- tldr page: https://tldr.inbrowser.app/pages/linux/wipefs

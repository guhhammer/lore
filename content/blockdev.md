# blockdev

Manage, query, and manipulate block devices.

## Usage
- `sudo blockdev --report` : print a report for all devices
- `sudo blockdev --report /dev/sdXY` : print a report for a specific device
- `sudo blockdev --getsz /dev/sdXY` : get the size of a device in 512-byte sectors
- `sudo blockdev --setro /dev/sdXY` : set read-only
- `sudo blockdev --setrw /dev/sdXY` : set read-write
- `sudo blockdev --flushbufs /dev/sdXY` : flush buffers
- `sudo blockdev --getpbsz /dev/sdXY` : get the physical block size
- `sudo blockdev --setra 128 /dev/sdXY` : set the read-ahead value to 128 sectors

## Categories
users, hardware, databases

## Links
- More information: https://manned.org/blockdev
- tldr page: https://tldr.inbrowser.app/pages/linux/blockdev

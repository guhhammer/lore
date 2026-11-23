# jmtpfs

FUSE-based filesystem for accessing MTP devices.

## Usage
- `jmtpfs path/to/directory` : mount an MTP device to a directory
- `jmtpfs -o allow_other,auto_unmount path/to/directory` : set mount options
- `jmtpfs -l` : list available MTP devices
- `jmtpfs -device=bus_id,device_id path/to/directory` : if multiple devices are present, mount a specific device
- `fusermount -u path/to/directory` : unmount MTP device

## Categories
files, hardware, disks

## Links
- More information: https://manned.org/jmtpfs
- tldr page: https://tldr.inbrowser.app/pages/common/jmtpfs

# f3fix

Edit the partition table of a fake flash drive.

## Usage
- `sudo f3fix /dev/device_name` : fill a fake flash drive with a single partition that matches its real capacity
- `sudo f3fix -b /dev/device_name` : mark the partition as bootable
- `sudo f3fix -f filesystem_type /dev/device_name` : specify the filesystem

## See also
- f3probe
- f3write
- f3read

## Categories
disks, databases, users

## Links
- More information: https://manned.org/f3fix
- tldr page: https://tldr.inbrowser.app/pages/common/f3fix

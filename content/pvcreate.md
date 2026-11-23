# pvcreate

Initialize a disk or partition for use as a physical volume.

## Usage
- `sudo pvcreate /dev/sdXY` : initialize the `/dev/sda1` volume for use by LVM
- `sudo pvcreate -f /dev/sdXY` : force the creation without any confirmation prompts

## See also
- lvm

## Categories
disks

## Links
- More information: https://manned.org/pvcreate
- tldr page: https://tldr.inbrowser.app/pages/linux/pvcreate

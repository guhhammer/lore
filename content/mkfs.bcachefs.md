# mkfs.bcachefs

Create a `bcachefs` filesystem inside a partition.

## Usage
- `sudo mkfs.bcachefs /dev/sdXY` : create a `bcachefs` filesystem inside partition `Y` on a device `X`
- `sudo mkfs.bcachefs -L volume_label /dev/sdXY` : create a `bcachefs` filesystem with a volume label

## See also
- bcachefs

## Categories
danger, disks

## Links
- More information: https://bcachefs-docs.readthedocs.io/en/latest/mgmt-formatting.html
- tldr page: https://tldr.inbrowser.app/pages/linux/mkfs.bcachefs

# lvm

Manage physical volumes, volume groups, and logical volumes using the Logical Volume Manager (LVM) interactive shell.

## Usage
- `sudo lvm` : start the Logical Volume Manager interactive shell
- `sudo lvm pvcreate /dev/sdXY` : initialize a drive or partition to be used as a physical volume
- `sudo lvm pvdisplay` : display information about physical volumes
- `sudo lvm vgcreate vg1 /dev/sdXY` : create a volume group called vg1 from the physical volume on `/dev/sdXY`
- `sudo lvm vgdisplay` : display information about volume groups
- `sudo lvm lvcreate -L 10G vg1` : create a logical volume with size 10G from volume group vg1
- `sudo lvm lvdisplay` : display information about logical volumes
- `lvm help command` : display help for a specific command

## Categories
disks, users, desktop

## Links
- More information: https://manned.org/lvm
- tldr page: https://tldr.inbrowser.app/pages/linux/lvm

# vgchange

Change the attributes of a Logical Volume Manager (LVM) volume group.

## Usage
- `sudo vgchange -a y|n` : change the activation status of logical volumes in all volume groups
- `sudo vgchange -a y|n volume_group` : change the activation status of logical volumes in the specified volume group (determine with `vgscan`)

## See also
- lvm

## Categories
disks, users

## Links
- More information: https://manned.org/vgchange
- tldr page: https://tldr.inbrowser.app/pages/linux/vgchange

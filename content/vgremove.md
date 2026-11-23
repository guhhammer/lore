# vgremove

Remove volume group(s) in LVM.

## Usage
- `sudo vgremove volume_group` : remove a volume group with confirmation
- `sudo vgremove -f volume_group` : forcefully remove a volume group without confirmation
- `sudo vgremove -d -d volume_group` : set the debug level for detailed logging to level 2, (repeat `--debug` up to 6 times to increase the level)
- `sudo vgremove --config 'global/locking_type=1' volume_group` : use a specific config setting to override defaults
- `vgremove -h` : display help

## Categories
danger, users, disks

## Links
- More information: https://manned.org/vgremove
- tldr page: https://tldr.inbrowser.app/pages/linux/vgremove

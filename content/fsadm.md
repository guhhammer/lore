# fsadm

Check or resize a filesystem on a device.

## Usage
- `fsadm check /dev/vg_name/lv_name` : check a filesystem for errors
- `fsadm -n resize /dev/vg_name/lv_name 10G` : simulate resizing to a specific size, without making any changes
- `fsadm resize /dev/vg_name/lv_name` : grow a filesystem to fill the entire device (omit the size)
- `fsadm -l resize /dev/vg_name/lv_name 100G` : resize the filesystem and the underlying logical volume together
- `fsadm -e resize /dev/vg_name/lv_name 20G` : for ext2/3/4, unmount and resize offline

## Categories
disks, hardware

## Links
- More information: https://manned.org/fsadm
- tldr page: https://tldr.inbrowser.app/pages/linux/fsadm

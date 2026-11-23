# lvchange

Change attributes or the activation state of logical volumes.

## Usage
- `sudo lvchange -a y /dev/vg_name/lv_name` : activate a logical volume
- `sudo lvchange -a n /dev/vg_name/lv_name` : deactivate a logical volume
- `sudo lvchange -a ay /dev/vg_name/lv_name` : enable autoactivation for a logical volume
- `sudo lvchange -p r /dev/vg_name/lv_name` : set a logical volume to read-only (use `rw` for read-write)
- `sudo lvchange -k y /dev/vg_name/lv_name` : skip activation for a logical volume
- `sudo lvchange --refresh /dev/vg_name/lv_name` : refresh a logical volume using the latest metadata

## Categories
disks, users

## Links
- More information: https://manned.org/lvchange
- tldr page: https://tldr.inbrowser.app/pages/linux/lvchange

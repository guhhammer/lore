# lvrename

Rename a logical volume.

## Usage
- `sudo lvrename /dev/vg_name/old_lv /dev/vg_name/new_lv` : rename an LV using full paths
- `sudo lvrename vg_name old_lv new_lv` : rename an LV using the volume group and names
- `sudo lvrename -y /dev/vg_name/old_lv /dev/vg_name/new_lv` : answer "yes" to any prompts

## Categories
files, disks, users

## Links
- More information: https://manned.org/lvrename
- tldr page: https://tldr.inbrowser.app/pages/linux/lvrename

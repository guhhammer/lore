# lvconvert

Convert or modify the type, redundancy, or state of logical volumes.

## Usage
- `sudo lvconvert --type raid1 -m 1 /dev/vg_name/lv_name` : convert a linear logical volume to RAID1 (add one mirror: two copies total)
- `sudo lvconvert -m 0 /dev/vg_name/lv_name` : remove mirroring and convert back to a linear logical volume
- `sudo lvconvert --merge /dev/vg_name/snapshot_lv` : merge a snapshot back into its origin logical volume (applies on next activation)
- `sudo lvconvert --repair /dev/vg_name/lv_name` : repair a degraded RAID logical volume
- `sudo lvconvert --type thin-pool --poolmetadata /dev/vg_name/pool_metadata_lv /dev/vg_name/pool_lv` : convert an existing logical volume into a thin pool with a separate metadata LV

## Categories
disks, users

## Links
- More information: https://manned.org/lvconvert
- tldr page: https://tldr.inbrowser.app/pages/linux/lvconvert

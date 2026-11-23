# pvesm add

Add storages.

## Usage
- `pvesm ad dir storage_name --path path/to/directory` : add a directory storage
- `pvesm ad lvm storage_name --vgname volume_group_name` : add an LVM storage
- `pvesm ad lvmthin storage_name --vgname volume_group_name --thinpool logical_volume_name` : add an LVM-Thin storage

## Categories
files, disks

## Links
- More information: https://pve.proxmox.com/pve-docs/pvesm.1.html#cli_pvesm_add
- tldr page: https://tldr.inbrowser.app/pages/linux/pvesm-add

# lvm_import_vdo

Import a VDO volume created by the VDO manager into an LVM-managed logical volume (irreversible).

## Usage
- `lvm_import_vdo /dev/mapper/vdo_volume` : import a VDO volume with automatic names for the VG/LV
- `lvm_import_vdo -n vg_name/lv_name /dev/mapper/vdo_volume` : import and set the destination VG/LV name
- `lvm_import_vdo --dry-run /dev/mapper/vdo_volume` : simulate the import, showing what would be done without changing anything
- `lvm_import_vdo --no-snapshot /dev/mapper/vdo_volume` : convert in place without using a temporary snapshot (less safe)
- `lvm_import_vdo -v -y /dev/mapper/vdo_volume` : convert a VDO volume with verbose output and automatically answer "yes" to prompts
- `lvm_import_vdo --vdo-config path/to/vdo.conf /dev/mapper/vdo_volume` : use a VDO manager configuration file during import

## Categories
disks

## Links
- More information: https://manned.org/lvm_import_vdo
- tldr page: https://tldr.inbrowser.app/pages/linux/lvm_import_vdo

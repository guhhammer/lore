# vgcfgrestore

Restore volume group configuration (not user data) from a text back up file produced by `vgcfgbackup`.

## Usage
- `sudo vgcfgrestore vg_name` : restore VG metadata from last backup
- `sudo vgcfgrestore -f path/to/file vg_name` : restore VG metadata from specified backup-file
- `sudo vgcfgrestore -l vg_name` : list all VG metadata backups
- `sudo vgcfgrestore -l -f path/to/file vg_name` : list one VG metadata backup file

## Categories
users, files, disks

## Links
- More information: https://manned.org/vgcfgrestore
- tldr page: https://tldr.inbrowser.app/pages/linux/vgcfgrestore

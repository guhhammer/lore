# vgcfgbackup

Back up volume group configuration metadata to files (not user data).

## Usage
- `sudo vgcfgbackup` : back up metadata for all volume groups (to `/etc/lvm/backup/` by default)
- `sudo vgcfgbackup vg_name` : back up metadata for a specific volume group
- `sudo vgcfgbackup -f path/to/backup vg_name` : write the backup to a specific file
- `sudo vgcfgbackup -f /tmp/vg-backup-%s vg1 vg2 ...` : back up multiple VGs using a filename template (`%s` becomes the VG name)
- `sudo vgcfgbackup -v vg_name` : increase verbosity (repeat `-v` for more detail)

## Categories
users, disks

## Links
- More information: https://manned.org/vgcfgbackup
- tldr page: https://tldr.inbrowser.app/pages/linux/vgcfgbackup

# pmount

Mount arbitrary hotpluggable devices as a normal user.

## Usage
- `pmount /dev/to/block/device` : mount a device below `/media/` (using device as mount point)
- `pmount -t filesystem /dev/to/block/device label` : mount a device with a specific filesystem type to `/media/label`
- `pmount -t iso9660 -r /dev/cdrom` : mount a CD-ROM (filesystem type ISO9660) in read-only mode
- `pmount -t ntfs -w /dev/sdX` : mount an NTFS-formatted disk, forcing read-write access
- `pmount` : display all mounted removable devices

## Categories
disks, hardware, users

## Links
- More information: https://manned.org/pmount
- tldr page: https://tldr.inbrowser.app/pages/linux/pmount

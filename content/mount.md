# mount

Get access to an entire filesystem in one directory.

## Usage
- `mount` : show all mounted filesystems
- `mount path/to/device_file path/to/target_directory` : mount a device to a directory
- `mount -m path/to/device_file path/to/target_directory` : create a specific directory if it does not exist and mount a device to it
- `mount -o uid=user_id,gid=group_id path/to/device_file path/to/target_directory` : mount a device to a directory for a specific user
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : mount a CD-ROM device (with the filetype ISO9660) to `/cdrom` (readonly)
- `mount -a` : mount all the filesystems defined in `/etc/fstab`
- `mount path/to/mount_point` : mount a specific filesystem described in `/etc/fstab` (e.g. `/dev/sda1 /path/to/mount_point ext2 defaults 0 2`)
- `mount -B path/to/old_directory path/to/new_directory` : mount a directory to another directory

## See also
- udisksctl

## Categories
disks

## Links
- More information: https://manned.org/mount.8
- tldr page: https://tldr.inbrowser.app/pages/linux/mount

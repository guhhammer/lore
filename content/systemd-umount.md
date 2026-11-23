# systemd-umount

This command is an alias of `systemd-mount --umount`.

## Usage
- `systemd-mount path/to/file_or_device` : mount a filesystem (image or block device) at `/run/media/system/LABEL` where LABEL is the filesystem label or the device name if there is no label
- `systemd-mount path/to/file_or_device path/to/mount_point` : mount a filesystem (image or block device) at a specific location
- `systemd-mount --list` : list all local, known block devices with filesystems that may be mounted
- `systemd-mount --automount yes path/to/file_or_device` : create an automount point that mounts the actual filesystem at the time of first access
- `systemd-mount -u path/to/mount_point_or_device1 path/to/mount_point_or_device2 ...` : unmount one or more devices
- `systemd-mount -t file_system_type path/to/file_or_device path/to/mount_point` : mount a filesystem (image or block device) with a specific filesystem type
- `systemd-mount -o mount_options path/to/file_or_device path/to/mount_point` : mount a filesystem (image or block device) with additional mount options

## Categories
disks, system, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-umount

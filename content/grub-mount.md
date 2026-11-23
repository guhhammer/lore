# grub-mount

Mount a filesystem or filesystem image read-only using GRUB's filesystem drivers.

## Usage
- `grub-mount /dev/sdXY /mnt` : mount a block device or filesystem image to a mount point
- `grub-mount -r 2 disk.img /mnt` : mount an entire disk image's second partition, `-r` specifies the partition number in the image
- `grub-mount -C /dev/sdXY /mnt` : mount an encrypted device and prompt for a passphrase
- `grub-mount -K /path/to/zfs.key /dev/sdX /mnt` : load a ZFS encryption key from a file
- `grub-mount -d string image /mnt` : show debugging output for a matching category
- `grub-mount -v image /mnt` : enable verbose output
- `grub-mount -?` : display help
- `grub-mount --version` : display version

## Categories
disks, containers, media

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-mount

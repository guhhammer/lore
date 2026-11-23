# mkntfs

Create an NTFS file system.

## Usage
- `mkntfs [options] device [number-of-sectors]` : the general form, as the manual gives it
- `mkntfs -f` : perform quick (fast) format
- `mkntfs -L` : set the volume label for the filesystem
- `mkntfs -C` : enable compression on the volume
- `mkntfs -s` : specify the size of sectors in bytes

## Categories
disks, files

## Links
- Documentation: https://github.com/tuxera/ntfs-3g/wiki/

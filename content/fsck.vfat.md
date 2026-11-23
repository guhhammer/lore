# fsck.vfat

Check and repair MS-DOS FAT filesystems.

## Usage
- `fsck.fat [OPTIONS] DEVICE` : the general form, as the manual gives it
- `fsck.vfat -b` : make read-only boot sector check
- `fsck.vfat -c PAGE` : use DOS codepage PAGE to decode short file names
- `fsck.vfat -d PATH` : delete the specified file
- `fsck.vfat -f` : salvage unused cluster chains to files

## Categories
files

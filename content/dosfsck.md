# dosfsck

Check and repair MS-DOS FAT filesystems.

## Usage
- `fsck.fat [OPTIONS] DEVICE` : the general form, as the manual gives it
- `dosfsck -b` : make read-only boot sector check
- `dosfsck -c PAGE` : use DOS codepage PAGE to decode short file names
- `dosfsck -d PATH` : delete the specified file
- `dosfsck -f` : salvage unused cluster chains to files

## Categories
files

# fsck.msdos

Check and repair MS-DOS FAT filesystems.

## Usage
- `fsck.fat [OPTIONS] DEVICE` : the general form, as the manual gives it
- `fsck.msdos -b` : make read-only boot sector check
- `fsck.msdos -c PAGE` : use DOS codepage PAGE to decode short file names
- `fsck.msdos -d PATH` : delete the specified file
- `fsck.msdos -f` : salvage unused cluster chains to files

## Categories
files

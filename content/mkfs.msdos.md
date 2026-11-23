# mkfs.msdos

Create an MS-DOS FAT filesystem.

## Usage
- `mkfs.fat [OPTIONS] DEVICE [BLOCK-COUNT]` : the general form, as the manual gives it
- `mkfs.msdos -c` : check the device for bad blocks before creating the filesystem
- `mkfs.msdos -f NUMBER-OF-FATS` : specify the number of file allocation tables in the filesystem
- `mkfs.msdos -I` : ignore and disable safety checks
- `mkfs.msdos -l FILENAME` : read the bad blocks list from FILENAME

## Categories
danger, disks

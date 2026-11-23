# mkdosfs

Create an MS-DOS FAT filesystem.

## Usage
- `mkfs.fat [OPTIONS] DEVICE [BLOCK-COUNT]` : the general form, as the manual gives it
- `mkdosfs -c` : check the device for bad blocks before creating the filesystem
- `mkdosfs -f NUMBER-OF-FATS` : specify the number of file allocation tables in the filesystem
- `mkdosfs -I` : ignore and disable safety checks
- `mkdosfs -l FILENAME` : read the bad blocks list from FILENAME

## Categories
disks

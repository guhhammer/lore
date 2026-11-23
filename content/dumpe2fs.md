# dumpe2fs

Print the super block and blocks group information for ext2/ext3/ext4 filesystems. Unmount the partition before running this command using `umount device`.

## Usage
- `dumpe2fs /dev/sdXN` : display ext2, ext3, and ext4 filesystem information
- `dumpe2fs -b /dev/sdXN` : display the blocks which are reserved as bad in the filesystem
- `dumpe2fs -f /dev/sdXN` : force display filesystem information even with unrecognizable feature flags
- `dumpe2fs -h /dev/sdXN` : only display the superblock information and not any of the block group descriptor detail information
- `dumpe2fs -x /dev/sdXN` : print the detailed group information block numbers in hexadecimal format

## Categories
disks, users, desktop

## Links
- More information: https://manned.org/dumpe2fs
- tldr page: https://tldr.inbrowser.app/pages/linux/dumpe2fs

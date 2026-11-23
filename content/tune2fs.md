# tune2fs

Adjust parameters of an ext2, ext3, or ext4 filesystem. May be used on mounted filesystems.

## Usage
- `sudo tune2fs -c 2 /dev/sdXN` : set the max number of counts before a filesystem is checked to 2
- `sudo tune2fs -L 'MY_LABEL' /dev/sdXN` : set the filesystem label to `MY_LABEL`
- `sudo tune2fs -o discard,user_xattr /dev/sdXN` : enable discard and user-specified extended attributes for a filesystem
- `sudo tune2fs -o has_journal /dev/sdXN` : enable journaling for a filesystem
- `sudo tune2fs -U random /dev/sdXN` : assign a new randomly-generated UUID to a filesystem

## Categories
disks, users

## Links
- More information: https://manned.org/tune2fs
- tldr page: https://tldr.inbrowser.app/pages/linux/tune2fs

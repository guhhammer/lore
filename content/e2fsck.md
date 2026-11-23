# e2fsck

Check a Linux ext2/ext3/ext4 filesystem. The partition should be unmounted.

## Usage
- `sudo e2fsck /dev/sdXN` : check filesystem, reporting any damaged blocks
- `sudo e2fsck -p /dev/sdXN` : check filesystem and automatically repair (preen) any damaged blocks
- `sudo e2fsck -c /dev/sdXN` : check filesystem in read only mode
- `sudo e2fsck -f /dev/sdXN` : force checking even if the filesystem seems clean
- `sudo e2fsck -fccky /dev/sdXN` : perform an exhaustive, non-destructive read-write test for bad blocks and blacklist them

## Categories
disks, users

## Links
- More information: https://manned.org/e2fsck
- tldr page: https://tldr.inbrowser.app/pages/linux/e2fsck

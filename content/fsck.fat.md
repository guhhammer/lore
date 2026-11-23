# fsck.fat

Check and repair a FAT filesystem. The filesystem should be unmounted.

## Usage
- `sudo fsck.fat /dev/sdXY` : check a filesystem and interactively repair any problems
- `sudo fsck.fat -n /dev/sdXY` : check a filesystem without changing anything (no-op)
- `sudo fsck.fat -a /dev/sdXY` : automatically repair a filesystem
- `sudo fsck.fat -nl /dev/sdXY` : check a filesystem and list path names of files being processed
- `sudo fsck.fat -at /dev/sdXY` : automatically repair a filesystem and test for bad clusters

## Categories
disks, users

## Links
- More information: https://manned.org/fsck.fat
- tldr page: https://tldr.inbrowser.app/pages/linux/fsck.fat

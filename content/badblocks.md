# badblocks

Search a device for bad blocks. Some usages of badblocks can cause destructive actions, such as erasing all data on a disk, including the partition table.

## Usage
- `sudo badblocks /dev/sdX` : search a disk for bad blocks by using a non-destructive read-only test
- `sudo badblocks -n /dev/sdX` : search an unmounted disk for bad blocks with a non-destructive read-write test
- `sudo badblocks -w /dev/sdX` : search an unmounted disk for bad blocks with a destructive write test
- `sudo badblocks -svw /dev/sdX` : use the destructive write test and show verbose progress
- `sudo badblocks -o path/to/file -w /dev/sdX` : in destructive mode, output found blocks to a file
- `sudo badblocks -w -b 4096 -c 65536 /dev/sdX` : use the destructive mode with improved speed using 4K block size and 64K block count

## Categories
disks, users, development

## Links
- More information: https://manned.org/badblocks
- tldr page: https://tldr.inbrowser.app/pages/common/badblocks

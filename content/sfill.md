# sfill

Securely overwrite the free space and inodes of the partition where the specified directory resides.

## Usage
- `sfill /path/to/mounted_disk_directory` : overwrite free space and inodes of a disk with 38 writes (slow but secure)
- `sfill -l -v /path/to/mounted_disk_directory` : overwrite free space and inodes of a disk with 6 writes (fast but less secure) and show status (verbose)
- `sfill -ll -v /path/to/mounted_disk_directory` : overwrite free space and inodes of a disk with 1 write (very fast but less secure verbose) and show status
- `sfill -I /path/to/mounted_disk_directory` : overwrite only free space of a dIsk
- `sfill -i /path/to/mounted_disk_directory` : overwrite only free inodes of a disk

## Categories
disks, files, security

## Links
- More information: https://manned.org/sfill
- tldr page: https://tldr.inbrowser.app/pages/linux/sfill

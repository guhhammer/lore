# fallocate

Reserve or deallocate disk space to files. The utility allocates space without zeroing.

## Usage
- `fallocate -l 700M path/to/file` : reserve a file taking up 700 MiB of disk space
- `fallocate -c -l 200MB path/to/file` : shrink an already allocated file by 200 MB
- `fallocate -c -o 100M -l 20M path/to/file` : shrink 20 MB of space after 100 MiB in a file

## Categories
files, disks

## Links
- More information: https://manned.org/fallocate
- tldr page: https://tldr.inbrowser.app/pages/linux/fallocate

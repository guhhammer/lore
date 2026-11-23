# compsize

Calculate the compression ratio of a set of files on a btrfs filesystem.

## Usage
- `sudo compsize path/to/file_or_directory` : calculate the current compression ratio for a file or directory
- `sudo compsize -x path/to/file_or_directory` : don't traverse filesystem boundaries
- `sudo compsize -b path/to/file_or_directory` : show raw byte counts instead of human-readable sizes

## See also
- btrfs filesystem

## Categories
files, disks, users

## Links
- More information: https://manned.org/compsize
- tldr page: https://tldr.inbrowser.app/pages/linux/compsize

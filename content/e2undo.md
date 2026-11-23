# e2undo

Replay undo logs for an ext2/ext3/ext4 filesystem. This can be used to undo a failed operation by an e2fsprogs program.

## Usage
- `e2undo -h path/to/undo_file /dev/sdXN` : display information about a specific undo file
- `e2undo -nv path/to/undo_file /dev/sdXN` : simulate replaying the undo file and display the candidate blocks
- `e2undo path/to/undo_file /dev/sdXN` : perform an undo operation
- `e2undo -v path/to/undo_file /dev/sdXN` : perform an undo operation and display verbose information
- `e2undo -z path/to/file.e2undo path/to/undo_file /dev/sdXN` : write the old contents of the block to an undo file before overwriting a filesystem block

## Categories
files, disks, desktop

## Links
- More information: https://manned.org/e2undo
- tldr page: https://tldr.inbrowser.app/pages/linux/e2undo

# e2image

Save critical ext2/ext3/ext4 filesystem metadata to a file.

## Usage
- `e2image /dev/sdXN path/to/image_file` : write metadata located on device to a specific file
- `e2image /dev/sdXN -` : print metadata located on device to `stdout`
- `e2image -I /dev/sdXN path/to/image_file` : restore the filesystem metadata back to the device
- `e2image -r /dev/sdXN path/to/image_file` : create a large raw sparse file with metadata at proper offsets
- `e2image -Q /dev/sdXN path/to/image_file` : create a QCOW2 image file instead of a normal or raw image file

## Categories
files, disks, hardware

## Links
- More information: https://manned.org/e2image
- tldr page: https://tldr.inbrowser.app/pages/linux/e2image

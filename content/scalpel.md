# scalpel

Recover files from a disk image based on file headers and footers. File patterns must be enabled in the configuration file (all are commented out by default).

## Usage
- `scalpel -c path/to/scalpel.conf -o path/to/output_directory path/to/disk_image.dd` : carve files from a disk image using a specific configuration file
- `scalpel -p -c path/to/scalpel.conf -o path/to/output_directory path/to/disk_image.dd` : preview which files would be carved, without actually carving them
- `scalpel -s number -c path/to/scalpel.conf -o path/to/output_directory path/to/disk_image.dd` : skip the first N bytes of the disk image before carving
- `scalpel -O -c path/to/scalpel.conf -o path/to/output_directory path/to/disk_image.dd` : carve files without organizing them into subdirectories by type
- `scalpel -h` : display help
- `scalpel -V` : display version

## Categories
files, containers, disks

## Links
- More information: https://manned.org/scalpel
- tldr page: https://tldr.inbrowser.app/pages/linux/scalpel

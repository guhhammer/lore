# mkisofs

Create ISO files from directories. Also aliased as `genisoimage`.

## Usage
- `mkisofs -o filename.iso path/to/source_directory` : create an ISO from a directory
- `mkisofs -o filename.iso -V "label_name" path/to/source_directory` : set the disc label when creating an ISO
- `mkisofs -o filename.iso -allow-limited-size path/to/source_directory` : create an ISO image with files larger than 2GiB by reporting a smaller apparent size for ISO9660 filesystems

## Categories
files

## Links
- More information: https://manned.org/mkisofs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkisofs

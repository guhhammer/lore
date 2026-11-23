# lvmdump

Collect diagnostic information about LVM2 (Logical Volume Manager). By default, outputs a compressed `.tar` archive with system and configuration data in the home directory.

## Usage
- `lvmdump` : generate a basic dump
- `lvmdump -a -l -m` : generate an extended dump with metadata and daemon info
- `lvmdump -d path/to/directory` : dump the information into a directory instead of a tarball
- `lvmdump -h` : display help

## Categories
archives, files, disks

## Links
- More information: https://manned.org/lvmdump
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmdump

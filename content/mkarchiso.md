# mkarchiso

Build Arch Linux ISO images from an archiso profile.

## Usage
- `mkarchiso -v /usr/share/archiso/configs/releng` : build an ISO from the official `releng` profile
- `mkarchiso -v -w path/to/work_directory -o path/to/output_directory path/to/profile_directory` : build an ISO with custom working and output directories
- `mkarchiso -r -o path/to/output_directory path/to/profile_directory` : build an ISO and delete the working directory afterwards
- `mkarchiso -m "bootstrap netboot" path/to/profile_directory` : build a bootstrap tarball and netboot artifacts instead of an ISO
- `mkarchiso -L label -P "publisher" -A "application" path/to/profile_directory` : set the volume label, publisher, and application name of the ISO
- `mkarchiso -p "package1 package2 ..." path/to/profile_directory` : install additional packages into the image
- `mkarchiso -h` : display help

## Categories
files, development, containers

## Links
- More information: https://manned.org/mkarchiso
- tldr page: https://tldr.inbrowser.app/pages/linux/mkarchiso

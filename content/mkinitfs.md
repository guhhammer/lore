# mkinitfs

Generate an initramfs on Alpine Linux.

## Usage
- `mkinitfs` : generate an initramfs with the features specified in `/etc/mkinitfs/mkinitfs.conf`
- `mkinitfs -c path/to/config` : use a different configuration file
- `mkinitfs -C gzip|xz|zstd|lz4|none` : compress the initramfs using the specified compression algorithm (default: `gzip`)
- `mkinitfs -l` : list files that will be included in the initramfs image
- `mkinitfs -L` : list all available features

## Categories
archives

## Links
- More information: https://manned.org/mkinitfs
- tldr page: https://tldr.inbrowser.app/pages/linux/mkinitfs

# mkswap

Set up a Linux swap area on a device or in a file. Note: `path/to/file` can either point to a regular file or a swap partition.

## Usage
- `sudo mkswap path/to/file` : set up a given swap area
- `sudo mkswap -c path/to/file` : check a partition for bad blocks before creating the swap area
- `sudo mkswap -L label /dev/sdXY` : specify a label for the partition (to allow `swapon` to use the label)
- `sudo mkswap -U clear|random|time|uuid_value` : use the specified UUID
- `sudo mkswap -s file_size -F path/to/swapfile` : set up a swap file (for btrfs, see `tldr btrfs filesystem` instead)

## Categories
danger, files, disks, users

## Links
- More information: https://manned.org/mkswap
- tldr page: https://tldr.inbrowser.app/pages/linux/mkswap

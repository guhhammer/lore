# swapon

Enable devices and files for swapping. Note: `path/to/file` can either point to a regular file or a swap partition.

## Usage
- `swapon` : show swap information
- `sudo swapon path/to/file` : enable a given swap area
- `sudo swapon -a` : enable all swap areas specified in `/etc/fstab` except those with the `noauto` option
- `sudo swapon -L label` : enable a swap partition by its label

## Categories
disks, files, hardware

## Links
- More information: https://manned.org/swapon.8
- tldr page: https://tldr.inbrowser.app/pages/linux/swapon

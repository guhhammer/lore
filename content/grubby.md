# grubby

Tool for configuring `grub` and `zipl` bootloaders.

## Usage
- `sudo grubby --update-kernel=ALL --args 'quiet console=ttyS0'` : add kernel boot arguments to all kernel menu entries
- `sudo grubby --update-kernel=DEFAULT --remove-args quiet` : remove existing arguments from the entry for the default kernel
- `sudo grubby --info=ALL` : list all kernel menu entries

## Categories
system, users

## Links
- More information: https://manned.org/grubby.8
- tldr page: https://tldr.inbrowser.app/pages/linux/grubby

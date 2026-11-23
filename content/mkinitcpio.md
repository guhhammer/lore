# mkinitcpio

Generate initial ramdisk environments for booting the Linux kernel based on the specified preset(s).

## Usage
- `mkinitcpio` : simulate generating the initial ramdisk environments and print what would be done
- `sudo mkinitcpio -P` : generate ramdisk environments based on all existing presets (used to regenerate all the initramfs images after a change in `/etc/mkinitcpio.conf`)
- `sudo mkinitcpio -p linux` : generate a ramdisk environment based on the `linux` preset
- `sudo mkinitcpio -p linux-lts` : generate a ramdisk environment based on the `linux-lts` preset
- `sudo mkinitcpio -c path/to/mkinitcpio.conf -g path/to/initramfs.img` : generate an initramfs image using an alternative configuration file
- `sudo mkinitcpio -k kernel_version -g path/to/initramfs.img` : generate an initramfs image for a kernel other than the one currently running (the installed kernel releases can be found in `/usr/lib/modules/`)
- `mkinitcpio -L` : list all available hooks
- `mkinitcpio -H hook_name` : display help for a specific hook

## Categories
system, users, files

## Links
- More information: https://manned.org/mkinitcpio
- tldr page: https://tldr.inbrowser.app/pages/linux/mkinitcpio

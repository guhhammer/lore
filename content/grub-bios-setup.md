# grub-bios-setup

Set up a device to use GRUB with a BIOS configuration. You should use `grub-install` instead of `grub-bios-setup` in most cases.

## Usage
- `grub-bios-setup /dev/sdX` : set up a device to boot with GRUB
- `grub-bios-setup -f /dev/sdX` : install even if problems are detected
- `grub-bios-setup -d /boot/grub /dev/sdX` : install GRUB in a specific directory

## Categories
hardware, packages

## Links
- More information: https://manned.org/grub-bios-setup
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-bios-setup

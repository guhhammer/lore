# grub-install

Install GRUB to a device.

## Usage
- `sudo grub-install path/to/device` : install GRUB on a BIOS system
- `sudo grub-install --target i386-pc path/to/device` : install GRUB on a BIOS system while specifying architecture
- `sudo grub-install --efi-directory path/to/efi_directory` : install GRUB on a UEFI system
- `sudo grub-install --target x86_64-efi --efi-directory path/to/efi_directory --bootloader-id GRUB` : install GRUB on a UEFI system while specifying architecture and boot menu text
- `sudo grub-install --target x86_64-efi --efi-directory path/to/efi_directory --modules "part_gpt part_msdos"` : install GRUB pre-loading specific modules
- `grub-install -?` : display help

## Categories
packages, files, hardware

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-install

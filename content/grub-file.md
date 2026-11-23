# grub-file

Check if a file is of a bootable image type.

## Usage
- `grub-file --is-arm-efi path/to/file` : check if a file is an ARM EFI image
- `grub-file --is-i386-efi path/to/file` : check if a file is an i386 EFI image
- `grub-file --is-x86_64-efi path/to/file` : check if a file is an x86_64 EFI image
- `grub-file --is-arm-linux path/to/file` : check if a file is an ARM image (Linux kernel)
- `grub-file --is-x86-linux path/to/file` : check if a file is an x86 image (Linux kernel)
- `grub-file --is-x86_64-xnu path/to/file` : check if a file is an x86_64 XNU image (macOS kernel)

## Categories
files, containers, media

## Links
- More information: https://manned.org/grub-file
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-file

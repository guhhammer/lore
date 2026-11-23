# grub-mkimage

Make a bootable image of GRUB.

## Usage
- `grub-mkimage [OPTION...] [OPTION]... [MODULES]` : the general form, as the manual gives it
- `grub-mkimage -c` : embed FILE as an early config
- `grub-mkimage --disable-shim-lock` : disable shim_lock verifier
- `grub-mkimage -d` : use images and modules under DIR [default=/usr/lib/grub/<platform>]
- `grub-mkimage -D` : embed FILE as a device tree (DTB)

## Categories
containers, media

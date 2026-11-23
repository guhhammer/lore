# grub-mkstandalone

Make a memdisk-based GRUB image.

## Usage
- `grub-mkstandalone [OPTION...] [OPTION] SOURCE...` : the general form, as the manual gives it
- `grub-mkstandalone --compress=no|xz|gz|lzo` : compress GRUB files [optional]
- `grub-mkstandalone --disable-shim-lock` : disable shim_lock verifier
- `grub-mkstandalone --dtb=FILE` : embed a specific DTB
- `grub-mkstandalone -d` : use images and modules under DIR [default=/usr/lib/grub/<platform>]

## Categories
containers, media, archives

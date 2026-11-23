# grub-mknetdir

Prepare a GRUB netboot directory.

## Usage
- `grub-mknetdir [OPTION...]` : the general form, as the manual gives it
- `grub-mknetdir --compress=no|xz|gz|lzo` : compress GRUB files [optional]
- `grub-mknetdir --disable-shim-lock` : disable shim_lock verifier
- `grub-mknetdir --dtb=FILE` : embed a specific DTB
- `grub-mknetdir -d` : use images and modules under DIR [default=/usr/lib/grub/<platform>]

## Categories
files, archives

# lsinitrd

Show the contents of an initramfs image.

## Usage
- `lsinitrd` : show the contents of the initramfs image for the current kernel
- `lsinitrd -k kernel_version` : show the contents of the initramfs image for the specified kernel
- `lsinitrd path/to/initramfs.img` : show the contents of the specified initramfs image
- `lsinitrd -m` : list modules included in the initramfs image
- `lsinitrd --unpack` : unpack the initramfs to the current directory

## See also
- dracut

## Categories
containers, media

## Links
- More information: https://github.com/dracutdevs/dracut/blob/master/man/lsinitrd.1.asc
- tldr page: https://tldr.inbrowser.app/pages/linux/lsinitrd

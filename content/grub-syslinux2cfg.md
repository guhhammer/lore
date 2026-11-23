# grub-syslinux2cfg

Transform syslinux config into grub.cfg.

## Usage
- `grub-syslinux2cfg [OPTION...] FILE` : the general form, as the manual gives it
- `grub-syslinux2cfg -i` : assume input is an isolinux configuration file
- `grub-syslinux2cfg -o` : write output to FILE [default=stdout]
- `grub-syslinux2cfg -p` : assume input is a pxelinux configuration file
- `grub-syslinux2cfg -r` : root directory of the syslinux disk [default=/]

## Categories
files

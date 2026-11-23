# grub-editenv

Edit GRUB environment variables.

## Usage
- `grub-editenv /boot/grub/grubenv set default=Ubuntu` : set a default boot entry (Assuming the boot entry already exists)
- `grub-editenv /boot/grub/grubenv list` : display all GRUB environment variables
- `grub-editenv /boot/grub/grubenv unset saved_entry` : reset the `saved_entry` variable to the default

## Categories
system

## Links
- More information: https://www.gnu.org/software/grub/manual/grub/grub.html
- tldr page: https://tldr.inbrowser.app/pages/linux/grub-editenv

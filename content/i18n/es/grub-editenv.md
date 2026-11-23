# grub-editenv

Edita las variables de entorno de GRUB.

## Usage
- `grub-editenv /boot/grub/grubenv set default=Ubuntu` : establece una entrada de arranque por defecto (asumiendo que la entrada de arranque ya existe)
- `grub-editenv /boot/grub/grubenv list` : muestra todas las variables de entorno de GRUB
- `grub-editenv /boot/grub/grubenv unset saved_entry` : restablece la variable `saved_entry` al valor por defecto

## Links
- More information: https://www.gnu.org/software/grub/manual/grub/grub.html

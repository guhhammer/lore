# grub-editenv

Modifica le variabili d'ambiente di GRUB.

## Usage
- `grub-editenv /boot/grub/grubenv set default=Ubuntu` : imposta una voce di avvio predefinita (supponendo che la voce di avvio esista già)
- `grub-editenv /boot/grub/grubenv list` : mostra tutte le variabili d'ambiente di GRUB
- `grub-editenv /boot/grub/grubenv unset saved_entry` : reimposta la variabile `saved_entry` al valore predefinito

## Links
- More information: https://www.gnu.org/software/grub/manual/grub/grub.html

# grub-bios-setup

Configura un dispositivo per usare GRUB con una configurazione BIOS. Dovresti usare `grub-install` invece di `grub-bios-setup` nella maggior parte dei casi.

## Usage
- `grub-bios-setup /dev/sdX` : configura un dispositivo per l'avvio con GRUB
- `grub-bios-setup -f /dev/sdX` : installa anche se vengono rilevati problemi
- `grub-bios-setup -d /boot/grub /dev/sdX` : installa GRUB in una directory specifica

## Links
- More information: https://manned.org/grub-bios-setup

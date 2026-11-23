# grub-bios-setup

Configura un dispositivo para usar GRUB con una configuración BIOS. Se debe usar `grub-install` en lugar de `grub-bios-setup` en la mayoría de los casos.

## Usage
- `grub-bios-setup /dev/sdX` : configura un dispositivo para arrancar con GRUB
- `grub-bios-setup -f /dev/sdX` : instala incluso si se detectan problemas
- `grub-bios-setup -d /boot/grub /dev/sdX` : instala GRUB en un directorio específico

## Links
- More information: https://manned.org/grub-bios-setup

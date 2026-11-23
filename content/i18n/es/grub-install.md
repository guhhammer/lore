# grub-install

Instala GRUB en un dispositivo.

## Usage
- `sudo grub-install ruta/al/dispositivo` : instala GRUB en un sistema BIOS
- `sudo grub-install --target i386-pc ruta/al/dispositivo` : instala GRUB en un sistema BIOS especificando la arquitectura
- `sudo grub-install --efi-directory ruta/al/directorio_efi` : instala GRUB en un sistema UEFI
- `sudo grub-install --target x86_64-efi --efi-directory ruta/al/directorio_efi --bootloader-id GRUB` : instala GRUB en un sistema UEFI especificando la arquitectura y el texto del menú de arranque
- `sudo grub-install --target x86_64-efi --efi-directory ruta/al/directorio_efi --modules "part_gpt part_msdos"` : instala GRUB precargando módulos específicos
- `grub-install -?` : muestra la ayuda

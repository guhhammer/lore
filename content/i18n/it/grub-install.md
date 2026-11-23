# grub-install

Installa GRUB su un dispositivo.

## Usage
- `sudo grub-install path/to/device` : installa GRUB su un sistema BIOS
- `sudo grub-install --target i386-pc path/to/device` : installa GRUB su un sistema BIOS specificando l'architettura
- `sudo grub-install --efi-directory path/to/efi_directory` : installa GRUB su un sistema UEFI
- `sudo grub-install --target x86_64-efi --efi-directory path/to/efi_directory --bootloader-id GRUB` : installa GRUB su un sistema UEFI specificando architettura e testo del menu di avvio
- `sudo grub-install --target x86_64-efi --efi-directory path/to/efi_directory --modules "part_gpt part_msdos"` : installa GRUB pre-caricando moduli specifici
- `grub-install -?` : mostra l'aiuto

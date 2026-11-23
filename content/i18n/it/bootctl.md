# bootctl

Controlla le impostazioni di avvio del firmware EFI e gestisce il boot loader.

## Usage
- `bootctl` : mostra informazioni sul firmware di sistema e i boot loader
- `bootctl list` : mostra tutte le voci di boot loader disponibili
- `sudo bootctl reboot-to-firmware true` : imposta un flag per avviare nel firmware di sistema al prossimo riavvio (simile a `sudo systemctl reboot --firmware-setup`)
- `bootctl --esp-path /path/to/efi_system_partition/` : specifica il percorso alla partizione EFI di sistema (predefinito `/efi/`, `/boot/` o `/boot/efi`)
- `sudo bootctl install` : installa `systemd-boot` nella partizione EFI di sistema
- `sudo bootctl remove` : rimuove tutte le versioni installate di `systemd-boot` dalla partizione EFI di sistema

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/bootctl.html

# grub-install

Bir cihaza GRUB kurun.

## Usage
- `sudo grub-install cihaz/yolu` : bir BIOS sisteme GRUB kurun
- `sudo grub-install --target i386-pc cihaz/yolu` : GRUB'u, bir BIOS sistemine belirtilen mimari ile kurun
- `sudo grub-install --efi-directory efi/dizini/yolu` : bir UEFI sisteme GRUB kurun
- `sudo grub-install --target x86_64-efi --efi-directory efi/dizin/yolu --bootloader-id GRUB` : GRUB'u, UEFI bir sisteme belirtilen mimari ve belirtilen önyükleme menü yazısı ile kurun
- `sudo grub-install --target x86_64-efi --efi-directory efi/dizin/yolu --modules "part_gpt part_msdos"` : GRUB'u, yükleme öncesi özel modüller ile kurun
- `grub-install -?` : yardımı görüntüle

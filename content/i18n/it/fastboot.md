# fastboot

Comunica con il dispositivo Android connessione quando in modalità bootloader (la situazione in cui `adb` non funziona).

## Usage
- `fastboot oem unlock` : sblocca il bootloader
- `fastboot oem lock` : ri-blocca il bootloader
- `fastboot reboot bootloader` : riavvia il dispositivo da modalità fastboot, nuovamente in modalità fastboot
- `fastboot flash file.img` : esegue in Flash di una data immagine
- `fastboot flash recovery file.img` : esegue il Flash di una recovery image personalizzata
- `fastboot devices` : mostra i dispositivi connessi
- `fastboot getvar all` : mostra tutte le informazioni su un dispositivo

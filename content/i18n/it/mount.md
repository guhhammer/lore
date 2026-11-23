# mount

Fornisce accesso a un intero filesystem in una directory specifica.

## Usage
- `mount` : mostra tutti i filesystem montati
- `mount -t tipo_di_filesystem percorso/del/dispositivo percorso/della/directory_desiderata` : monta un dispositivo in una directory
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : monta un CD-ROM (con il filetypo ISO9660) a `/cdrom` (sola lettura)
- `mount -a` : monta tutti i filesystem definiti in `/etc/fstab`
- `mount /my_drive` : monta un filesystem specifico descritto in `/etc/fstab` (ad esempio `/dev/sda1 /my_drive ext2 defaults 0 2`)
- `mount -B percorso/della/vecchia_directory percorso/della/nuova_directory` : monta una directory in un'altra directory

## Links
- More information: https://manned.org/mount.8

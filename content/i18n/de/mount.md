# mount

Ermöglicht den Zugriff auf ein gesamtes Dateisystem in einem Verzeichnis.

## Usage
- `mount` : zeige alle eingehängten Dateisysteme
- `mount -t dateisystemtyp pfad/zu/gerätedatei pfad/zu/zielverzeichnis` : hänge ein Gerät in ein Verzeichnis ein
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : hänge ein CD-ROM-Gerät (Dateisystemtyp ISO9660) in das Verzeichnis `/cdrom` schreibgeschützt ein
- `mount -a` : hänge alle Dateisysteme ein, die in `/etc/fstab` definiert sind
- `mount /meine_platte` : hänge ein Dateisystem ein, das in `/etc/fstab` beschrieben ist (z. B. `/dev/sda1 /meine_platte ext2 defaults 0 2`)
- `mount -B pfad/zu/altem_verzeichnis pfad/zu/neuem_verzeichnis` : hänge ein Verzeichnis in ein anderes Verzeichnis ein (danach sind die Inhalte über beide Pfade verfügbar)

## Links
- More information: https://manned.org/mount.8

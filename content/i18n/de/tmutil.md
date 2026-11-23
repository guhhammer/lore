# tmutil

Dienstprogramm zum Verwalten von Time Machine-Backups. Die meisten Befehle erfordern Root-Rechte.

## Usage
- `sudo tmutil setdestination pfad/zu/einhänge_punkt` : setze ein HFS+ Laufwerk als Backupziel
- `sudo tmutil setdestination "protocol://benutzer:passwort@host/share"` : setze eine APF-Freigabe oder SMB-Freigabe als Backupziel
- `sudo tmutil setdestination -a ziel` : hänge das angegebene Ziel an die Liste der Backupziele an
- `sudo tmutil enable` : aktiviere automatische Backups
- `sudo tmutil disable` : deaktiviere automatische Backups
- `sudo tmutil startbackup` : starte ein Backup im Hintergrund, falls nicht bereits eines läuft
- `sudo tmutil startbackup -b` : starte ein Backup im Vordergrund, falls nicht bereits eines läuft
- `sudo tmutil stopbackup` : stoppe ein laufendes Backup

## Links
- More information: https://keith.github.io/xcode-man-pages/tmutil.8.html

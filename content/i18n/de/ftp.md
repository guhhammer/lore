# ftp

Interaktiver Dateitransfer zwischen einem lokalen und einem entfernten FTP-Server.

## Usage
- `ftp host` : verbinden mit einem entfernten FTP-Server
- `ftp -A host` : anmelden als anonymer Benutzer
- `ftp -n host` : deaktivieren der automatische Anmeldung bei der ersten Verbindung
- `ftp -s:pfad/zu/Datei host` : ausführen einer Datei, die eine Liste von FTP-Befehlen enthält
- `mget *.png` : herunterladen von mehrerern Dateien (globaler Ausdruck)
- `mput *.zip` : hochladen von mehrerern Dateien (globaler Ausdruck)
- `mdelete *.txt` : löschen mehrerer Dateien auf dem entfernten Server
- `ftp --help` : zeige Hilfe an

# chown

Ändere den Besitzer und die Besitzergruppe von Dateien und Verzeichnissen.

## Usage
- `chown benutzer pfad/zu/datei_oder_verzeichnis` : ändere den Besitzer einer Datei/eines Verzeichnisses
- `chown benutzer:gruppe pfad/zu/datei_oder_verzeichnis` : ändere den Besitzer und die Besitzergruppe einer Datei/eines Verzeichnisses
- `chown -R benutzer pfad/zu/verzeichnis` : ändere den Besitzer eines Verzeichnisses rekursiv
- `chown -h benutzer pfad/zu/symlink` : ändere den Besitzer eines symbolischen Links
- `chown --reference pfad/zu/referenzdatei_oder_verzeichnis pfad/zu/datei_oder_verzeichnis` : ändere den Besitzer einer Datei/eines Verzeichnisses, damit sie/es mit einer Referenzdatei übereinstimmt

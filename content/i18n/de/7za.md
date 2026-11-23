# 7za

Ein Dateiarchivierer mit hoher Kompressionsrate. Eine alleinstehende Version von `7z` mit Unterstützung für neuere Archivtypen.

## Usage
- `7za a pfad/zu/archiv.7z pfad/zu/datei_oder_verzeichnis` : archiviere eine Datei oder ein Verzeichnis
- `7za a pfad/zu/verschlüsselt.7z -ppasswort -mhe=on pfad/zu/archiv.7z` : verschlüssle ein vorhandenes Archiv (einschließlich Dateinamen)
- `7za x pfad/zu/archiv.7z` : extrahiere ein Archiv und behalte die originale Verzeichnisstruktur bei
- `7za x pfad/zu/archiv.7z -opfad/zu/verzeichnis` : extrahiere ein Archiv in ein bestimmtes Verzeichnis
- `7za x pfad/zu/archiv.7z -so` : extrahiere ein Archiv nach `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|zip pfad/zu/archiv.7z pfad/zu/datei_oder_verzeichnis` : archiviere mit einem bestimmten Archivtyp
- `7za l pfad/zu/archiv.7z` : liste den Inhalt einer Archivdatei auf

## Links
- More information: https://manned.org/7za

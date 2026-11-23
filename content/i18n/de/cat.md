# cat

Verkette und gib einzelne oder mehrere Dateien aus.

## Usage
- `cat pfad/zu/datei` : gib den Inhalt einer Datei aus
- `cat pfad/zu/datei1 pfad/zu/datei2 ... >pfad/zu/ziel_datei` : verkette mehrere Dateien und speichere das Ergebnis in einer neuen Datei
- `cat pfad/zu/datei1 pfad/zu/datei2 ... >>pfad/zu/ziel_datei` : verkette mehrere Dateien und hänge das Ergebnis an eine Datei an
- `cat -u /dev/tty12 >/dev/tty13` : kopiere den Inhalt einer Datei in eine Ausgabedatei ohne zu puffern
- `cat - >pfad/zu/datei` : schreibe `stdin` in eine Datei

## Links
- More information: https://manned.org/cat.1posix

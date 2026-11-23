# bat

Ausgabe und Verkettung von einzelnen Dateien. Ein `cat`-Ersatz mit Syntax-Hervorhebung und Git-Integration.

## Usage
- `bat pfad/zu/datei` : gib den Inhalt einer Datei in `stdout` aus
- `bat pfad/zu/datei1 pfad/zu/datei2 >pfad/zu/ziel_datei` : verkette mehrere Dateien zu einer Zieldatei
- `bat pfad/zu/datei1 pfad/zu/datei2 >>pfad/zu/ziel_datei` : hänge mehrere Dateien an eine Zieldatei an
- `bat -n pfad/zu/datei` : nummeriere alle ausgegebenen Zeilen
- `bat -l json pfad/zu/datei.json` : hebe die Syntax einer JSON-Datei hervor
- `bat -L` : zeige alle unterstützten Sprachen an

## Links
- More information: https://manned.org/bat

# apt-file

Suche nach Dateien in apt-Paketen, auch in den nicht-installierten.

## Usage
- `sudo apt update` : aktualisiere die Metadatenbank
- `apt-file find pfad/zu/datei` : suche nach Paketen, die die/den spezifizierten Pfad/Datei enthalten
- `apt-file list paketname` : liste die Inhalte eines bestimmten Pakets auf
- `apt-file find -x regular_expression` : suche nach Paketen auf die die `regex` zutrifft

## Links
- More information: https://manned.org/apt-file

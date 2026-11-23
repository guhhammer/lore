# addr2line

Konvertiere Adressen von Binärdateien in Dateinamen und Zeilennummern.

## Usage
- `addr2line -e pfad/zu/binärdatei adresse` : zeige den Dateinamen und die Zeilennummer des Quellcodes von einer Befehlsadresse einer ausführbaren Datei an
- `addr2line -e pfad/zu/binärdatei -f adresse` : zeige den Funktionsnamen, Dateinamen und Zeilennummer an
- `addr2line -e pfad/zu/binärdatei -f -C adresse` : entmangele den Funktionsnamen für C++ Code

## Links
- More information: https://manned.org/addr2line

# df

Verschafft einen Überblick über verfügbaren Speicherplatz im Dateisystem.

## Usage
- `df` : zeige verfügbaren Platz auf allen eingehängten Dateisystemen
- `df -h` : zeige verfügbaren Platz auf allen eingehängten Dateisystemen in einem menschenlesbaren Format
- `df pfad/zu/datei_oder_verzeichnis` : zeige das Dateisystem und dessen Speicherverbrauch, das die angegebene Datei oder Verzeichnis enthält
- `df -i` : zeige Statistiken über die Anzahl freier Inodes
- `df -x squashfs -x tmpfs` : zeige alle Dateisysteme bis auf die eines bestimmten Typs

## Links
- More information: https://manned.org/df.1posix

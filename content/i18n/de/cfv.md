# cfv

Command-line File Verify, eine Anwendung zum Testen und Erstellen von Prüfsummendateien.

## Usage
- `cfv -T` : teste Prüfsummendateien im aktuellen Verzeichnis
- `cfv -Tvnu -f pfad/zu/datei` : teste mit ausführlicher Ausgabe v, benenne fehlerhafte Dateien um, und gib unverifizierte Dateien zurück. Verwende dazu eine bestimmte Prüfsummendatei f
- `cfv -C -t "sfv|sha256|md5|..." datei1 datei2 ...` : erstelle C eine Prüfsummendatei unter Verwendung eines bestimmten typs für die ausgewählten Dateien
- `cfv -Cr` : erstelle C separate Prüfsummendateien für jedes rekursiv ausgewählte Verzeichnis
- `cfv -C -f sums.sha256 -rr` : erstelle C eine einzelne `sha256`-Datei f für alle [rr]ekursiv ausgewählten Dateien
- `cfv -Czvl -p pfad/zu/verzeichnis` : erstelle C eine `gzip`-komprimierte `.sfv`-Datei mit ausführlicher Ausgabe v. Folge dabei `symlinks` und verwende `pfad/zu/verzeichnis` als Basispfad

## Links
- More information: https://manned.org/cfv

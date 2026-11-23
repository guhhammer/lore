# tar

Archivierungstool. Häufig kombiniert mit anderen Methoden zur Komprimierung, wie gzip oder bzip2.

## Usage
- `tar cf pfad/zu/ziel.tar pfad/zu/datei1 pfad/zu/datei2 ...` : erstelle ein Archiv von Dateien
- `tar czf ziel.tar.gz pfad/zu/datei1 pfad/zu/datei2 ...` : erstelle ein mit gzip komprimiertes Archiv
- `tar czf pfad/zu/ziel.tar.gz -C pfad/zu/verzeichnis .` : erstelle ein mit gzip komprimiertes Archiv mit relativen Pfaden
- `tar xvf pfad/zu/quelle.tar.ext` : extrahiere ein (komprimiertes) Archiv in das derzeitige Verzeichnis im ausführlichen Modus
- `tar xf pfad/zu/quelle.tar -C pfad/zu/verzeichnis` : extrahiere ein Archiv in ein Verzeichnis
- `tar caf pfad/zu/ziel.tar.xz pfad/zu/datei1 pfad/zu/datei2 ...` : erstelle ein komprimiertes Archiv und benutze den die Dateiendung des Archivs um die Kompressionsmethode zu wählen
- `tar tvf pfad/zu/quelle.tar` : führe die Inhalte eines tar Archivs auf
- `tar xf pfad/zu/quelle.tar --wildcards "*.html"` : extrahiere Dateien die mit einem Muster übereinstimmen

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html

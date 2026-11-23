# diff

Vergleiche Dateien und Verzeichnisse.

## Usage
- `diff pfad/zu/datei1 pfad/zu/datei2` : vergleiche Dateien (Listet jene Veränderungen auf, mit denen `datei1` zu `datei2` wird)
- `diff -w pfad/zu/datei1 pfad/zu/datei2` : vergleiche Dateien und ignoriere Leerzeichen
- `diff -y pfad/zu/datei1 pfad/zu/datei2` : vergleiche Dateien und zeige Unterschiede nebeneinander
- `diff -u pfad/zu/datei1 pfad/zu/datei2` : vergleiche Dateien und zeige Unterschiede in vereinheitlichtem Format (wie in `git diff`)
- `diff -r altes_verzeichnis neues_verzeichnis` : vergleiche Verzeichnisse rekursiv (zeigt sowohl Namen von unterschiedlichen Dateien/Verzeichnissen, als auch Unterschiede zwischen Dateien)
- `diff -r -q altes_verzeichnis neues_verzeichnis` : vergleiche Verzeichnisse und zeige nur die Namen der Dateien, die unterschiedlich sind
- `diff -a -u -N pfad/zu/datei1 pfad/zu/datei2 >pfad/zu/diff.patch` : erstelle ein patch-Datei für Git bestehend aus den Unterschieden zweier Dateien und behandle fehlende Dateien als leer

## Links
- More information: https://manned.org/diff

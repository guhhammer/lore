# find

Dateien oder Verzeichnisse in einem Verzeichnisbaum rekursiv suchen.

## Usage
- `find root_path -name '*.ext'` : dateien nach Erweiterung suchen
- `find root_path -path '*/path/*/*.ext' -or -name '*pattern*'` : suche Dateien, die mehreren Pfad-/Namensmustern entsprechen
- `find root_path -type d -iname '*lib*'` : suche Verzeichnisse, die ohne Berücksichtigung der Groß- und Kleinschreibung einem bestimmten Namensmustern entsprechen
- `find root_path -name '*.py' -not -path '*/site-packages/*'` : suche Dateien, die einem bestimmten Namensmustern entsprechen, unter Ausschluss bestimmter Pfade
- `find root_path -maxdepth 1 -size +500k -size -10M` : suche Dateien, die einem bestimmten Größenbereich entsprechen, wobei die rekursive Tiefe auf "1" begrenzt wird
- `find root_path -name '*.ext' -exec wc -l {} \;` : führe für jede Datei einen Befehl aus (verwende `{}` innerhalb des Befehls, um auf den Dateinamen zuzugreifen)
- `find root_path -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : finde alle heute geänderten Dateien und übergebe die Ergebnisse als Argumente an einen einzelnen Befehl
- `find root_path -type f|d -empty -delete -print` : suche leere Dateien oder Verzeichnisse, gebe diese aus und lösche diese

## Links
- More information: https://manned.org/find

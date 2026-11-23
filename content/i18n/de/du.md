# du

Disk usage: Plattenplatzverbrauch von Dateien und Verzeichnissen ermitteln.

## Usage
- `du -b|k|m pfad/zu/verzeichnis` : liste die Größe von Verzeichnissen und Unterverzeichnissen in den gegebenen Einheiten (B/KiB/MiB) auf
- `du -h pfad/zu/verzeichnis` : liste die Größe von Verzeichnissen und Unterverzeichnissen in menschenlesbaren Einheiten auf (d.h. automatische Auswahl der geeigneten Einheit für jede Größe)
- `du -sh pfad/zu/verzeichnis` : zeige die Größe eines einzelnen Verzeichnisses in menschenlesbaren Einheiten
- `du -ah pfad/zu/verzeichnis` : liste die Größe von Verzeichnissen und Unterverzeichnissen und aller ihrer Dateien in menschenlesbaren Einheiten auf
- `du -h -d N pfad/zu/verzeichnis` : liste die menschenlesbaren Größen eines Verzeichnisses und aller Unterverzeichnisse, bis zu einer Tiefe von `N` Ebenen
- `du -ch */*.jpg` : liste die menschenlesbare Größe aller `.jpg`-Dateien in Unterverzeichnissen des aktuellen Verzeichnisses auf und zeige am Ende die kumulierte Gesamtsumme an

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html

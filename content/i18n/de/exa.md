# exa

Ein moderner Ersatz für `ls` (Verzeichnisinhalte auflisten).

## Usage
- `exa -1` : liste eine Datei pro Zeile auf
- `exa -a` : liste alle Dateien auf, einschließlich versteckter Dateien
- `exa -l -a` : liste alle Dateien im langen Format auf (Berechtigungen, Eigentümer, Größe und Änderungsdatum)
- `exa -r -s size` : liste Dateien nach Größe absteigend sortiert auf
- `exa -l -T -L 3` : zeige Dateien in einer Baumstruktur an, die drei Ebenen tief ist
- `exa -l -s modified` : liste Dateien nach Änderungsdatum aufsteigend sortiert auf
- `exa -l -h --icons --git` : liste Dateien inklusive Header, Icons und Git-Status
- `exa --git-ignore` : liste keine Dateien auf, die in `.gitignore` erwähnt werden

## Links
- More information: https://github.com/ogham/exa#command-line-options

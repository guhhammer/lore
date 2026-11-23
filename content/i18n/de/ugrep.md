# ugrep

Ultraschnelles Suchtool mit Abfrage-TUI.

## Usage
- `ugrep -Q` : starte eine interaktive TUI um rekursiv nach Dateien im aktuellen Verzeichnis zu suchen (`<Ctrl z>` für Hilfe)
- `ugrep "suchmuster"` : suche im aktuellen Verzeichnis rekursiv nach Dateien, die einem bestimmten regulären Ausdruck entsprechen
- `ugrep -n "suchmuster" pfad/zu/datei_oder_verzeichnis` : suche in einer Datei oder in allen Dateien in einem bestimmten Verzeichnis und zeige die Zeilennummer jedes Treffers
- `ugrep -l "suchmuster"` : suche in allen Dateien im aktuellen Verzeichnis rekursiv und zeige den Dateinamen jeder passenden Datei
- `ugrep -Z3 "suchmuster"` : suche nach einem "fuzzy" regulären Ausdruck mit bis zu 3 zusätzlichen, fehlenden oder nicht übereinstimmenden Zeichen
- `ugrep -z "suchmuster"` : suche auch in allen komprimierten Dateien und Zip- und `.tar`-Archive
- `ugrep -g "glob_muster" "suchmuster"` : suche nur in Dateien deren Dateinamen mit einem bestimmten `glob`-Muster übereinstimmen
- `ugrep -t cpp "suchmuster"` : suche nur in C++ Quelldateien (verwende `--file-type=list`, um mögliche Optionen aufzulisten)

## Links
- More information: https://github.com/Genivia/ugrep#man-page

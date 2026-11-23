# tail

Gib das Ende einer Datei aus.

## Usage
- `tail -n anzahl_an_zeilen datei` : zeige die letzten Zeilen einer Datei an
- `tail -n +zeile datei` : zeige alle Zeilen einer Datei ab einer bestimmten Zeile an
- `tail -n anzahl_an_bytes datei` : zeige die letzten Bytes einer Datei an
- `tail -f datei` : lies aus einer Datei, bis `<Ctrl c>` gedrückt wird
- `tail -F datei` : lies aus einer Datei, bis `<Ctrl c>` gedrückt wird, selbst, wenn die Datei nicht zugänglich ist
- `tail -n anzahl_an_zeilen -s anzahl_an_sekunden -f datei` : zeige die letzten Zeilen einer Datei an und lade alle paar Sekunden neu

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html

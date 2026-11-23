# vimdiff

Öffne zwei oder mehr Dateien in Vim und zeige ihre Unterschiede an.

## Usage
- `vimdiff pfad/zu/datei_1 pfad/zu/datei_2` : öffne zwei Dateien und zeige ihre Unterschiede an
- `<Ctrl w><h>|<l>` : bewege den Cursor zum linken|rechten Fenster
- `<[><c>` : springe zum vorigen Unterschied
- `<]><c>` : springe zum nächsten Unterschied
- `<d><o>` : kopiere die hervorgehobenen Unterschiede vom anderen in das aktuelle Fenster
- `<d><p>` : kopiere die hervorgehobenen Unterschiede vom aktuellen in das andere Fenster
- `<:>diffupdate` : aktualisiere die hervorgehobenen Unterschiede und Textfaltungen
- `<z><a>` : öffne/Schließe die Textfaltung unter dem Cursor

## Links
- More information: https://www.vim.org/

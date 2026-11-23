# cut

Schneide Felder von `stdin` oder einer Datei aus.

## Usage
- `befehl | cut --characters|fields 1|1,10|1-10|1-|-10` : schneide bestimmte Zeichen oder einen Bereich von Feldern jeder Zeile aus
- `befehl | cut --delimiter "," --fields 1` : schneide einen bestimmten Bereich von Feldern jeder Zeile mit einem bestimmten Trennzeichen aus
- `cut --characters 1 pfad/zu/datei` : schneide einen bestimmten Bereich von Zeichen jeder Zeile einer bestimmten Datei aus

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html

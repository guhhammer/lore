# abbr

Abkürzungen für die fish shell verwalten. Die vom Nutzer definierten Abkürzungen werden nach der Eingabe durch die Langversionen ersetzt.

## Usage
- `abbr -a abkürzungsname befehl befehlsparameter` : neue Abkürzung hinzufügen
- `abbr --rename alter_name neuer_name` : vorhandene Abkürzung umbenennen
- `abbr -e abkürzungsname` : vorhandene Abkürzung löschen
- `ssh host_name abbr -s | source` : abkürzungen eines anderen Host über SSH importieren

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html

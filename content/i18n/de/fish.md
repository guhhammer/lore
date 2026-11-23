# fish

The Friendly Interactive SHell. Eine benutzerfreundliche Eingabeaufforderung.

## Usage
- `fish` : starte eine interaktive Shell-Sitzung
- `fish -N` : starte eine interaktive Shell-Sitzung ohne die Start-Konfiguration zu laden
- `fish -c "echo 'fish wird ausgeführt'"` : führe einen bestimmten Befehl aus
- `fish pfad/zu/skript.fish` : führe ein bestimmtes Skript aus
- `fish -N pfad/zu/skript.fish` : überprüfe ein bestimmtes Skript auf Syntaxfehler
- `fish -P` : starte eine private, interaktive Shell-Sitzung, in der fish weder auf die Shell-History zugreift, noch diese verändert
- `set -U -x variablenname variablenwert` : definiere und exportiere eine Umgebungsvariable, die über mehrere Shell-Neustarts hinweg existiert (builtin)

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html

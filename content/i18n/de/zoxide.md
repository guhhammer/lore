# zoxide

Behält den Überblick über die am häufigsten verwendeten Verzeichnisse. Verwendet einen Ranking-Algorithmus, um zum besten Treffer zu navigieren.

## Usage
- `zoxide query string` : wechsel zu dem Verzeichnis mit dem höchsten Rang, das `string` im Namen enthält
- `zoxide query string1 string2` : wechsel in das höchstrangige Verzeichnis, das `string1` und dann `string2` enthält
- `zoxide query -i` : starte eine interaktive Verzeichnissuche (erfordert `fzf`)
- `zoxide add path/to/directory` : füge ein Verzeichnis hinzu oder erhöhe seinen Rang
- `zoxide remove path/to/directory` : entferne ein Verzeichnis aus der Datenbank von `zoxide`
- `zoxide init bash|elvish|fish|nushell|posix|powershell|tcsh|xonsh|zsh` : generiere Shell-Konfigurationen für Befehls-Aliase (`z`, `zi`) für die angegebene Shell

## Links
- More information: https://manned.org/zoxide

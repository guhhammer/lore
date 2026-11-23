# alias

Erstellt Aliasse - Alterative Namen für Befehle. Aliasse laufen mit der aktuellen Shell-Sitzung ab, es sei denn, sie werden in der Konfigurationsdatei der Shell definiert, z.B. `~/.bashrc`.

## Usage
- `alias` : listet alle Aliasse auf
- `alias alias="befehl"` : erstellt einen Alias
- `alias alias` : zeigt den mit einem bestimmten Alias verknüpften Befehl an
- `unalias alias` : entferne einen Alias
- `alias rm="rm -i"` : macht `rm` zu einem interaktiven Befehl
- `alias la="ls -a"` : erstellt den Alias `la` für `ls -a`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias

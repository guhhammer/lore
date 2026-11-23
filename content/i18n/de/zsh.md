# zsh

Z SHell. Mit `bash` und `sh` kompatible Eingabeaufforderung.

## Usage
- `zsh` : starte eine interaktive Eingabeaufforderung
- `zsh -c "echo 'zsh wird ausgeführt'"` : führe Parameter als Befehl aus
- `zsh pfad/zu/skript.zsh` : führe Befehle aus einem Skript aus
- `zsh -n pfad/zu/skript.zsh` : prüfe ein Skript nach Syntaxfehlern, ohne es auszuführen
- `echo "echo 'zsh wird ausgeführt'" | zsh` : führe Befehle von `stdin` aus
- `zsh -x pfad/zu/skript.zsh` : führe Befehle aus einem Skript aus und schreibe die Befehle in die Konsole
- `zsh -v` : starte eine interaktive Eingabeaufforderung, in der jeder Befehl ausgegeben wird, bevor er ausgeführt wird
- `zsh -f` : starte Zsh, ohne Nutzerkonfigurationen zu laden (z.B. `~/.zshrc`)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation

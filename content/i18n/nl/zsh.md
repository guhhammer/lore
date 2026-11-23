# zsh

Z SHell, een Bash-compatibele commandoregel-interpreteerder.

## Usage
- `zsh` : start een interactieve shellsessie
- `zsh -c "echo 'zsh wordt uitgevoerd'"` : voer specifieke commando's uit
- `zsh pad/naar/script.zsh` : voer een specifiek script uit
- `zsh -n pad/naar/script.zsh` : controleer een specifiek script op syntax fouten zonder het uit te voeren
- `echo "echo 'zsh wordt uitgevoerd'" | zsh` : voer specifieke commando's uit van `stdin`
- `zsh -x pad/naar/script.zsh` : voer een specifiek script uit en toon elke opdracht in het script voordat deze wordt uitgevoerd
- `zsh -v` : start een interactieve shellsessie in verbose modus en toon elke opdracht voordat deze wordt uitgevoerd
- `zsh -f` : start Zsh zonder het laden van user-level configuratie (bijv. `~/.zshrc`)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation

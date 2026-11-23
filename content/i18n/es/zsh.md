# zsh

Z SHell, un intérprete de línea de comandos compatible con Bash.

## Usage
- `zsh` : comienza una sesión interactiva en el intérprete de comandos
- `zsh -c "comando"` : ejecuta un comando y sale
- `zsh ruta/al/script.zsh` : ejecuta un script
- `zsh --no-exec ruta/al/script.zsh` : comprueba si hay errores de sintaxis en un script sin ejecutarlo
- `comando | zsh` : ejecuta comandos desde `stdin`
- `zsh --xtrace ruta/al/script.zsh` : ejecuta un script, mostrando cada comando antes de ejecutarlo
- `zsh --verbose` : comienza una sesión interactiva en el intérprete de comandos en modo detallado, mostrando cada comando antes de ejecutarlo
- `noglob command` : ejecuta un comando dentro de `zsh` con patrones `glob` desactivados

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Invocation.html#Invocation

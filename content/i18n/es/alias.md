# alias

Crea alias -- palabras que son reemplazadas por una cadena de comando(s). Los alias son temporales en la sesión de shell actual, a no ser que estén definidos en el archivo de configuración de la shell, ej. `~/.bashrc`.

## Usage
- `alias` : lista todos los aliases
- `alias nombre="comando"` : crea un alias genérico
- `alias nombre` : muestra el comando asociado a un alias
- `unalias nombre` : elimina un alias (con el comando asociado)
- `alias rm="rm -i"` : convierte `rm` en un comando interactivo
- `alias la="ls -a"` : define `la` como un atajo para `ls -a`

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-alias

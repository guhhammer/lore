# rargs

Ejecuta un comando por cada línea de entrada estándar. Similar a `xargs`, pero con soporte de coincidencia de patrones.

## Usage
- `comando | rargs comando {0}` : ejecuta un comando por cada línea de entrada, como `xargs` (`{0}` indica donde sustituir en el texto)
- `comando | rargs -e comando {0}` : imprime los comandos que se ejecutarían en vez de ejecutarlos
- `comando | rargs -p '(.*).bak' mv {0} {1}` : elimina la extensión `.bak` de todos los archivos de una lista
- `comando | rargs -w máxima_cantidad_de_procesos` : ejecuta comandos en paralelo
- `comando | rargs -0 comando {0}` : lee líneas de entrada separadas por el caracter `NUL` (`\0`) en vez de `LF` (`\n`)

## Links
- More information: https://github.com/lotabout/rargs

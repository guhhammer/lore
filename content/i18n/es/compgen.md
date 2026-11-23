# compgen

Comando integrado de Bash para generar posibles coincidencias de finalización en Bash. Se utiliza normalmente en funciones de finalización personalizadas.

## Usage
- `compgen -c` : enumera todos los comandos integrados, alias, funciones y ejecutables del intérprete de comandos que se pueden ejecutar
- `compgen -V COMPREPLY -c str` : enumera todos los comandos que puede ejecutar que comienzan con una cadena específica y guarda los resultados en `$COMPREPLY`
- `compgen -W "apple orange banana" a` : compara con una lista de palabras
- `compgen -a` : enumera todos los alias
- `compgen -A function` : enumera todas las funciones que se pueden ejecutar
- `compgen -k` : muestra las palabras clave reservadas del shell
- `compgen -ac ls` : vea todos los comandos/alias disponibles que comienzan por `ls`
- `compgen -u` : enumera todos los usuarios del sistema

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-compgen

# export

Exporta variables de un intérprete de comandos (shell) a procesos hijos.

## Usage
- `export VARIABLE=valor` : establece una variable de entorno
- `export -n VARIABLE` : quita una variable de entorno
- `export -f NOMBRE_FUNCION` : exporta una función a los procesos hijos
- `export PATH=$PATH:ruta/a/añadir` : añade una ruta a la variable de ambiente `$PATH`
- `export -p` : muestra una lista de variables exportadas activas en forma de comando de interfaz de comandos (shell command form)

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-export

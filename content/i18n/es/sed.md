# sed

Edita texto de manera scriptable.

## Usage
- `comando | sed 's/apple/mango/g'` : reemplaza todas las ocurrencias de `apple` (`regex` básica) por `mango` (`regex` básica) en todas las líneas de entrada y muestra el resultado en `stdout`
- `comando | sed -f ruta/al/script.sed` : ejecuta un archivo de script específico y muestra el resultado en `stdout`
- `comando | sed -n '1p'` : imprime solo la primera línea en `stdout`

## Links
- More information: https://manned.org/sed.1posix

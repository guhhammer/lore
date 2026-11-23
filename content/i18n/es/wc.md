# wc

Cuenta líneas, palabras, y bytes.

## Usage
- `wc -l ruta/al/archivo` : cuenta todas las líneas en un archivo
- `wc -w ruta/al/archivo` : cuenta todas las palabras en un archivo
- `wc -c ruta/al/archivo` : cuenta todos los bytes en un archivo
- `wc -m ruta/al/archivo` : cuenta todos los caracteres en un archivo (considerando los caracteres de varios bytes)
- `find . | wc` : cuenta todas las líneas, palabras y bytes desde `stdin`
- `wc -L ruta/al/archivo` : cuenta la longitud de la línea más larga en número de caracteres

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html

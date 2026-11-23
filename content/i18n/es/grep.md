# grep

Encuentra patrones en archivos usando expresiones regulares.

## Usage
- `grep "patrón_de_búsqueda" ruta/al/archivo` : busca un patrón en un archivo
- `grep -F "cadena_exacta" ruta/al/archivo` : busca una cadena de caracteres específica (la cadena no será interpretada como una `regex`)
- `grep -rnI "patrón_de_búsqueda" ruta/al/directorio` : busca un patrón en todos los archivos de forma recursiva en un directorio, mostrando los números de línea de las coincidencias e ignorando los archivos binarios
- `grep -Ei "patrón_de_búsqueda" ruta/al/archivo` : utiliza expresiones regulares extendidas (los metacaracteres `?`, `+`, `{}`, `()` y `|` no requieren de una barra inversa), sin distinguir entre mayúsculas y minúsculas
- `grep --context|--before-context|--after-context 3 "patrón_de_búsqueda" ruta/al/archivo` : imprime 3 líneas alrededor, antes o después de cada coincidencia
- `grep -Hn --color=always "patrón_de_búsqueda" ruta/al/archivo` : imprime con colores el nombre del archivo y el número de línea de cada coincidencia
- `grep -o "patrón_de_búsqueda" ruta/al/archivo` : busca líneas que coincidan con un patrón e imprime solo el texto coincidente
- `cat ruta/al/archivo | grep -v "patrón_de_busqueda"` : busca líneas en`stdin` que no coincidan con el patrón

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html

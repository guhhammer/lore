# bzgrep

Encuentra patrones en archivos comprimidos `bzip2` usando `grep`.

## Usage
- `bzgrep "patrón_de_búsqueda" ruta/al/archivo` : busca un patrón dentro de un archivo comprimido
- `bzgrep -r "patrón_de_búsqueda" ruta/al/archivo_tar` : busca un patrón de forma recursiva en un archivo comprimido bzip2 `.tar`
- `bzgrep --context|--before-context|--after-context 3 "patrón_de_búsqueda" ruta/al/archivo` : imprime 3 líneas de Contexto alrededor, Antes o Después de cada coincidencia
- `bzgrep -H -n "patrón_de_búsqueda" ruta/al/archivo` : imprime el nombre del archivo y el número de línea de cada coincidencia
- `bzgrep -o "patrón_de_búsqueda" ruta/al/archivo` : busca líneas que coincidan con un patrón, imprimiendo solo el texto coincidente
- `cat ruta/al/archivo_comprimido | bzgrep -v "patrón_de_búsqueda"` : busca en `stdin` líneas que no coincidan con un patrón
- `bzgrep -E -i "patrón_de_búsqueda" ruta/al/archivo` : utiliza `regex` extendida (admite `?`, `+`, `{}`, `()` y `|`), sin distinguir mayúsculas de minúsculas

## Links
- More information: https://manned.org/bzgrep

# xzgrep

Busca archivos posiblemente comprimidos con `xz`, `lzma`, `gzip`, `bzip2`, `lzop`, o `zstd` utilizando expresiones regulares.

## Usage
- `xzgrep "patrón_de_búsqueda" ruta/al/archivo` : busca un patrón dentro de un archivo
- `xzgrep --fixed-strings "cadena_exacta" ruta/al/archivo` : busca una cadena exacta (sin expresiones regulares)
- `xzgrep --line-number "patrón_de_búsqueda" ruta/al/archivo` : busca un patrón en todos los archivos mostrando los números de línea que coinciden
- `xzgrep --extended-regexp --ignore-case "patrón_de_búsqueda" ruta/al/archivo` : utiliza expresiones regulares extendidas (soporta `?`, `+`, `{}`, `()` y `|`), sin diferenciar mayúsculas y minúsculas (case-insensitive)
- `xzgrep --context|before-context|after-context=3 "patrón_de_búsqueda" ruta/al/archivo` : imprime 3 líneas de contexto alrededor, antes o después de cada coincidencia
- `xzgrep --with-filename --line-number --color=always "patrón_de_búsqueda" ruta/al/archivo` : imprime el nombre del archivo y número de línea para cada coincidencia en color
- `xzgrep --only-matching "patrón_de_búsqueda" ruta/al/archivo` : busca líneas que coincidan con un patrón, imprime solo el texto coincidente

## Links
- More information: https://manned.org/xzgrep

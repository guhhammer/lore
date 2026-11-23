# zgrep

Busca patrones de texto en archivos contenidos dentro de archivos comprimidos.

## Usage
- `zgrep patrón ruta/al/archivo_comprimido` : busca un patrón en un archivo comprimido (distingue entre mayúsculas y minúsculas)
- `zgrep --context|--before-context|--after-context 3 patrón ruta/al/archivo_comprimido` : muestra 3 líneas de Contexto alrededor, Before o After de cada coincidencia
- `zgrep -i patrón ruta/al/archivo_comprimido` : busca un patrón en un archivo comprimido (sin distinguir mayúsculas y minúsculas)
- `zgrep -c patrón ruta/al/archivo_comprimido` : muestra el recuento de líneas que contienen el patrón coincidente en un archivo comprimido
- `zgrep -v patrón ruta/al/archivo_comprimido` : muestra las líneas que no contienen el patrón (invertir la función de búsqueda)
- `zgrep -e "patrón_1" -e "patrón_2" ruta/al/archivo_comprimido` : busca varios patrones en un archivo comprimido
- `zgrep -E regex ruta/al/archivo` : usa expresiones regulares extendidas (admite `?`, `+`, `{}`, `()` y `|`)

## Links
- More information: https://manned.org/zgrep

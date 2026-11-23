# fd

Busca entradas en el sistema de archivos.

## Usage
- `fd "string|regex"` : busca de forma recursiva archivos que coincidan con un patrón específico en el directorio actual
- `fd "^string"` : busca archivos que comiencen con una cadena específica
- `fd -e txt` : busca archivos con una extensión específica
- `fd "string|regex" ruta/al/directorio` : busca archivos en un directorio específico
- `fd -H -I "string|regex"` : incluye archivos ignorados y ocultos en la búsqueda
- `fd string -E glob` : excluye archivos que coincidan con un patrón `glob` específico
- `fd "string|regex" -x comando` : ejecuta un comando en cada resultado de búsqueda devuelto
- `fd -d 1 "string|regex"` : busca archivos solo en el directorio actual

## Links
- More information: https://github.com/sharkdp/fd#command-line-options

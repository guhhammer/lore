# look

Muestra las líneas que empiezan por un prefijo en un archivo ordenado.

## Usage
- `look prefijo ruta/al/archivo` : busca líneas que comiencen con un prefijo específico en un archivo específico
- `look -f -d prefijo ruta/al/archivo` : búsqueda insensible a mayúsculas y minúsculas solo en caracteres alfanuméricos
- `look -t ,` : especifica un carácter de terminación de cadena (espacio por defecto)
- `look prefijo` : busca en `/usr/share/dict/words` (se asumen `--ignore-case` y `--alphanum`)

## Links
- More information: https://keith.github.io/xcode-man-pages/look.1.html

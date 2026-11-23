# rg

Ripgrep, una herramienta de búsqueda recursiva orientada a líneas. Aspira a ser una alternativa más rápida a `grep`.

## Usage
- `rg patrón` : busca de forma recursiva un patrón (`regex`) en el directorio actual
- `rg patrón ruta/al/archivo_o_directorio` : busca de forma recursiva un patrón en un archivo o directorio
- `rg -F -- cadena` : busca un patrón de cadena literal
- `rg -. --no-ignore patrón` : incluye los archivos ocultos y las entradas listadas en `.gitignore`
- `rg patrón -g 'patrón_glob_de_nombre_de_archivo'` : busca solo en los archivos que coinciden con los patrones `glob` (p. ej., `README.*`; use `!patrón_de_nombre_de_archivo` para excluirlos en su lugar)
- `rg --files | rg --passthru patrón` : lista de forma recursiva los nombres de archivo del directorio actual y resalta los que coinciden con un patrón
- `rg -l patrón` : lista solo los archivos con coincidencias (útil al redirigir la salida a otros comandos)
- `rg -v patrón` : muestra las líneas que no coinciden con el patrón

## Links
- More information: https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md

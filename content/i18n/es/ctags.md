# ctags

Genera un archivo de índice (o etiqueta) de objetos de lenguaje que se encuentran en los archivos de código fuente de muchos lenguajes de programación populares.

## Usage
- `ctags ruta/al/archivo` : genera etiquetas para un solo archivo y las envía a un archivo llamado "tags" en el directorio actual, sobrescribiendo el archivo si existe
- `ctags -f ruta/al/archivo *` : genera etiquetas para todos los archivos en el directorio actual y las envía a un archivo específico, sobrescribiendo el archivo si existe
- `ctags --recurse` : genera etiquetas para todos los archivos en el directorio actual y todos los subdirectorios
- `ctags --fields=+ne --output-format=json ruta/al/archivo` : genera etiquetas para un solo archivo y las envía con el número de línea inicial y el número de línea final en formato JSON

## Links
- More information: https://docs.ctags.io/en/latest/man/ctags.1.html

# alex

Una herramienta que detecta escritura insensible y desconsiderada. Ayuda a encontrar en el texto frases que son parciales con el género, que polarizan, o están relacionadas con la raza, son desconsideradas con la religión u otras frases tendenciosas.

## Usage
- `echo His network looks good | alex --stdin` : analiza texto desde `stdin`
- `alex` : analiza todos los archivos del directorio actual
- `alex ruta/al/archivo_de_texto.md` : analiza un archivo dado
- `alex *.md !ruta/hacia/ejemplo.md` : analiza todos los archivos Markdown excepto `ejemplo.md`

## Links
- More information: https://github.com/get-alex/alex

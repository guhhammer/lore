# latexpand

Simplifica los archivos fuente LaTeX eliminando comentarios y resolviendo `\include`s, `\input`s, etc.

## Usage
- `latexpand -o ruta/a/salida.tex ruta/al/archivo.tex` : simplifica el archivo fuente dado y guarda el resultado en el archivo de salida especificado
- `latexpand --keep-comments -o ruta/a/salida.tex ruta/al/archivo.tex` : no elimina los comentarios
- `latexpand --keep-includes -o ruta/a/salida.tex ruta/al/archivo.tex` : no expande `\include`s, `\input`s etc
- `latexpand --expand-usepackage -o ruta/a/salida.tex ruta/al/archivo.tex` : expande `\usepackage`s hasta encontrar los archivos STY correspondientes
- `latexpand --expand-bbl ruta/a/bibliografía.bbl -o ruta/a/salida.tex ruta/al/archivo.tex` : incorpora el archivo BBL especificado

## Links
- More information: https://www.ctan.org/pkg/latexpand

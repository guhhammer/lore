# asciidoctor

Un procesador que convierte archivos AsciiDoc a un formato publicable.

## Usage
- `asciidoctor ruta/al/archivo.adoc` : convierte un archivo `.adoc` específico a HTML (el formato de salida por defecto)
- `asciidoctor -a stylesheet=ruta/al/stylesheet.css ruta/al/archivo.adoc` : convierte un archivo `.adoc` específico a HTML y vincula una hoja de estilos CSS
- `asciidoctor -e ruta/al/archivo.adoc` : convierte un archivo específico `.adoc` en HTML incrustable, eliminando todo excepto el cuerpo
- `asciidoctor -b pdf -r asciidoctor-pdf ruta/al/archivo.adoc` : convierte un archivo `.adoc` dado en un PDF utilizando la biblioteca `asciidoctor-pdf`

## Links
- More information: https://docs.asciidoctor.org/asciidoctor/latest/cli/man1/asciidoctor/

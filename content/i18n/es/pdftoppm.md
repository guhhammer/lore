# pdftoppm

Convierte páginas de documentos PDF al formato de imagen Pixmap portátil.

## Usage
- `pdftoppm -f N -l M ruta/al/archivo.pdf prefijo_del_nombre_de_la_imagen` : especifica el rango de páginas a convertir (N es la primera página, M es la última página)
- `pdftoppm -singlefile ruta/al/archivo.pdf prefijo_del_nombre_de_la_imagen` : convierte solo la primera página de un PDF
- `pdftoppm -mono ruta/al/archivo.pdf prefijo_del_nombre_de_la_imagen` : genera un archivo PBM monocromático (en lugar de un archivo PPM de color)
- `pdftoppm -gray ruta/al/archivo.pdf prefijo_del_nombre_de_la_imagen` : genera un archivo PGM en escala de grises (en lugar de un archivo PPM de color)
- `pdftoppm -png ruta/al/archivo.pdf prefijo_del_nombre_de_la_imagen` : genera un archivo PNG en lugar de un archivo PPM

## Links
- More information: https://manned.org/pdftoppm

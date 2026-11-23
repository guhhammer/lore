# tar

Utilidad de archivado. A menudo se combina con un método de compresión, como gzip o bzip2.

## Usage
- `tar cf ruta/al/objetivo.tar ruta/al/archivo1 ruta/al/archivo2 ...` : crea un archivo y lo escribe en un fichero
- `tar czf ruta/al/objetivo.tar.gz ruta/al/archivo1 ruta/al/archivo2 ...` : crea un archivo con formato gzip y lo escribe en un fichero
- `tar czf ruta/al/objetivo.tar.gz -C ruta/al/directorio .` : crea un archivo con formato gzip desde un directorio utilizando rutas relativas
- `tar xvf ruta/a/fuente.tar.ext` : extrae un fichero (comprimido) al directorio actual verbosamente
- `tar xf ruta/al/archivo_de_entrada.tar.ext -C ruta/al/directorio` : extrae un fichero (comprimido) al directorio de destino
- `tar caf ruta/al/objetivo.tar.xz ruta/al/archivo1 ruta/al/archivo2 ...` : crea un archivo comprimido y lo escribe en una carpeta, utilizando la extensión del archivo para determinar automáticamente el programa de compresión
- `tar tvf ruta/al/archivo_de_entrada.tar` : lista el contenido de un fichero tar verbosamente
- `tar xf ruta/al/archivo_de_entrada.tar --wildcards "*.html"` : extrae ficheros que coincidan con un patrón de un fichero

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html

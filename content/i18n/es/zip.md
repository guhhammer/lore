# zip

Empaqueta y comprime (archiva) archivos en un archivo Zip.

## Usage
- `zip -r ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : añade archivos/directorios a un archivo específico
- `zip -d ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : elimina archivos/directorios de un archivo específico
- `zip -r ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ... -x ruta/a/archivos_o_directorios_excluidos` : archiva archivos/directorios excluyendo los especificados
- `zip -r -0..9 ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : archiva archivos/directorios con un nivel de compresión específico (`0` - el más bajo, `9` - el más alto)
- `zip -re ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : crea un archivo cifrado con una contraseña específica
- `zip -rs 3g ruta/a/archivo_comprimido.zip ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : archiva archivos/directorios en un archivo Zip dividido en varias partes (por ejemplo, partes de 3 GB)
- `zip -sf ruta/a/archivo_comprimido.zip` : imprime el contenido de un archivo específico

## Links
- More information: https://manned.org/zip

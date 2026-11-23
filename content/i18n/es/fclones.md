# fclones

Eficaz buscador y eliminador de archivos duplicados.

## Usage
- `fclones group .` : busca ficheros duplicados en el directorio actual
- `fclones group --cache ruta/al/directorio1 ruta/al/directorio2` : busca archivos duplicados en varios directorios y almacena los resultados en la caché
- `fclones group ruta/al/directorio --depth 1 >ruta/al/archivo.txt` : busca archivos duplicados solo en el directorio especificado, omitiendo los subdirectorios y guarda los resultados en un archivo
- `fclones <ruta/al/archivo.txt move ruta/al/directorio_objetivo` : mueve los archivos duplicados en un archivo de texto a un directorio diferente
- `fclones <ruta/al/archivo.txt link --soft --dry-run 2 >/dev/null` : simula un enlace simbólico a un archivo de texto sin realmente enlazarlo
- `fclones group . | fclones remove --priority newest` : elimina los archivos duplicados más recientes en el directorio actual sin almacenarlos en un archivo
- `fclones group . --name '*.jpg' -i --transform 'exiv2 -d a $IN' --in-place` : preprocesa los archivos JPEG en el directorio actual utilizando un comando externo para eliminar sus datos EXIF antes de buscar duplicados

## Links
- More information: https://github.com/pkolaczk/fclones#usage

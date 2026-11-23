# realpath

Muestra la ruta absoluta resuelta de un archivo o directorio.

## Usage
- `realpath ruta/al/archivo_o_directorio` : muestra la ruta absoluta de un archivo o directorio
- `realpath -e ruta/al/archivo_o_directorio` : requiere que existan todos los componentes de la ruta
- `realpath -L ruta/al/archivo_o_directorio` : resuelve los componentes `..` antes que los enlaces simbólicos
- `realpath -s ruta/al/archivo_o_directorio` : desactiva la expansión de enlaces simbólicos
- `realpath -q ruta/al/archivo_o_directorio` : suprime los mensajes de error

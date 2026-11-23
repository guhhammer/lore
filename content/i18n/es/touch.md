# touch

Crea archivos y establece los tiempos de acceso y modificación.

## Usage
- `touch ruta/al/archivo1 ruta/al/archivo2 ...` : crea los archivos especificados
- `touch -c -a|m ruta/al/archivo1 ruta/al/archivo2 ...` : establece los tiempos de acceso o modificación al momento actual y no crea un archivo si este no existe
- `touch -c -t YYYYMMDDHHMM.SS ruta/al/archivo1 ruta/al/archivo2 ...` : establece los tiempos de un archivo a un valor específico y no crea el archivo si no existe
- `touch -c -r ruta/al/archivo/de/referencia ruta/al/archivo1 ruta/al/archivo2 ...` : usa los tiempos de un archivo de referencia para establecer los tiempos en otro archivo y no crea el archivo si no existe

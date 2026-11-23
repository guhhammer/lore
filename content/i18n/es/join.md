# join

Une las líneas de dos archivos ordenados en un campo común.

## Usage
- `join ruta/al/archivo1 ruta/al/archivo2` : une dos archivos en el primer campo (por defecto)
- `join -t ',' ruta/al/archivo1 ruta/al/archivo2` : une dos archivos utilizando una coma (en lugar de un espacio) como separador de campos
- `join -1 3 -2 1 ruta/al/archivo1 ruta/al/archivo2` : une el campo3 del archivo1 con el campo1 del archivo2
- `join -a 1 ruta/al/archivo1 ruta/al/archivo2` : produce una línea por cada línea no emparejable del fichero1
- `cat ruta/al/archivo1 | join - ruta/al/archivo2` : une un fichero desde `stdin`

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/join-invocation.html

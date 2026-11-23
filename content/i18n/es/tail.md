# tail

Muestra las últimas líneas de un archivo de texto determinado.

## Usage
- `tail -n recuento ruta/al/archivo` : imprime las últimas líneas de 'recuento' de un archivo
- `tail -n +recuento ruta/al/archivo` : imprime un archivo desde una línea específica
- `tail -n recuento ruta/al/archivo` : imprime un número específico de bytes desde el final de algún archivo
- `tail -f ruta/al/archivo` : imprime las últimas líneas de un archivo en tiempo real hasta presionar `<Ctrl c>`
- `tail -F ruta/al/archivo` : mantiene leyendo las últimas líneas de un archivo hasta presionar `<Ctrl c>`, aunque el archivo sea inaccesible
- `tail -n recuento -s segundos -f ruta/al/archivo` : imprime las últimas líneas de 'recuento' en 'archivo' y se actualiza cada 'n' segundos

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html

# bfs

Búsqueda exhaustiva de tus archivos.

## Usage
- `bfs ruta/al/directorio -name '*.ext'` : busca archivos por extensión
- `bfs ruta/al/directorio -path '**/ruta/**/*.ext' -or -name '*patrón*'` : busca archivos que coincidan con varios patrones de ruta/nombre
- `bfs ruta/al/directorio -type d -iname '*lib*'` : busca directorios que coincidan con un nombre dado, sin distinguir mayúsculas de minúsculas
- `bfs ruta/al/directorio -name '*.py' -not -path '*/paquetes/*'` : busca archivos que coincidan con un patrón dado, excluyendo rutas específicas
- `bfs ruta/al/directorio -maxdepth 1 -size +500k -size -10M` : busca archivos que coincidan con un rango de tamaño dado, limitando la profundidad recursiva a "1"
- `bfs ruta/al/directorio -name '*.ext' -exec wc -l {} \;` : ejecuta un comando para cada archivo (utiliza `{}` dentro del comando para acceder al nombre del archivo)
- `bfs ruta/al/directorio -daystart -mtime -1 -exec tar -cvf archivo.tar {} \+` : busca todos los archivos modificados hoy y pasa los resultados a un único comando como argumentos
- `bfs ruta/al/directorio -type f|d -empty -delete -print` : encuentra archivos vacíos (0 bytes) o directorios y los elimina de forma detallada

## Links
- More information: https://manned.org/bfs

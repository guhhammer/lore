# find

Busca archivos o directorios bajo una jerarquía de directorios.

## Usage
- `find ruta/al/directorio -name '*.ext'` : encuentra archivos por extensión
- `find ruta/al/directorio -path '*/ruta/*/*.ext' -or -name '*patrón*'` : encuentra archivos que coinciden con múltiples patrones de ruta/nombre
- `find ruta/al/directorio -type d -iname '*lib*'` : encuentra directorios que coinciden con un nombre dado, en modo insensible a mayúsculas y minúsculas
- `find ruta/al/directorio -name '*.py' -not -path '*/site-packages/*'` : encuentra archivos que coinciden con un patrón dado, excluyendo rutas específicas
- `find ruta/al/directorio -maxdepth 1 -size +500k -size -10M` : encuentra archivos que coinciden con un rango de tamaño dado, limitando la profundidad recursiva a "1"
- `find ruta/al/directorio -name '*.ext' -exec wc -l {} \;` : ejecuta un comando para cada archivo (usa `{}` dentro del comando para acceder al nombre del archivo)
- `find ruta/al/directorio -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : encuentra todos los archivos modificados hoy y pasa los resultados a un solo comando como argumentos
- `find ruta/al/directorio -type f|d -empty -delete -print` : busca tanto archivos como directorios vacíos y los elimina detalladamente

## Links
- More information: https://manned.org/find

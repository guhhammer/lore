# comp

Compara el contenido de dos archivos o conjunto de archivos. Usa comodines (*) para comparar conjuntos de archivos.

## Usage
- `comp` : compara archivos de forma interactiva
- `comp ruta\al\archivo1 ruta\al\archivo2` : compara dos archivos específicos
- `comp ruta\al\directorio1\* ruta\al\directorio2\*` : compara dos conjuntos de archivos
- `comp /d ruta\al\archivo1 ruta\al\archivo2` : muestra las diferencias en formato decimal
- `comp /a ruta\al\archivo1 ruta\al\archivo2` : muestra las diferencias en formato aSCII
- `comp /l ruta\al\archivo1 ruta\al\archivo2` : muestra los números de línea para las diferencias
- `comp /c ruta\al\archivo1 ruta\al\archivo2` : compara archivos sin diferenciar mayúsculas y minúsculas (case-insensitive)
- `comp /n=5 ruta\al\archivo1 ruta\al\archivo2` : compara solo las primeras 5 líneas de cada archivo

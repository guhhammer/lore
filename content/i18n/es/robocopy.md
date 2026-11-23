# robocopy

Copia robusta de archivos y carpetas. Por defecto, los archivos solo se copiarán si la fuente y el destino tienen marcas de tiempo diferentes o tamaños de archivo distintos.

## Usage
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino *.jpg *.bmp` : copia todos los archivos `.jpg` y `.bmp` de un directorio a otro
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino /E` : copia todos los archivos y subdirectorios, incluidos los vacíos
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino /MIR /COPYALL` : espeja/sincroniza un directorio, eliminando todo lo que no esté en el origen e incluyendo todos los atributos y permisos
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino /E /XO` : copia todos los archivos y subdirectorios, excluyendo los archivos de origen que sean más antiguos que los archivos en el destino
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino /MIN:52428800 /L` : lista todos los archivos de 50 MB o más en lugar de copiarlos
- `robocopy ruta\al\directorio_origen ruta\al\directorio_destino /Z /R:5 /W:15` : permite reanudar si se pierde la conexión de red y limita los reintentos a 5 y el tiempo de espera a 15 segundos
- `robocopy /?` : muestra la ayuda

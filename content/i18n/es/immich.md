# immich

Immich tiene una interfaz de línea de comandos (CLI) que le permite realizar ciertas acciones desde la línea de comandos.

## Usage
- `immich login url_del_servidor/api clave_del_servidor` : autentica en el servidor de Immich
- `immich upload archivo1.jpg archivo2.jpg` : sube unas imágenes
- `immich upload -r ruta/al/directorio` : sube un directorio y sus subdirectorios
- `immich upload -r ruta/al/directorio -A "Vacaciones de verano"` : crea un álbum basado en un directorio
- `immich upload -r ruta/al/directorio -i **/Raw/** **/*.tif` : omite recursos que coincidan con un patrón global
- `immich upload -r ruta/al/directorio -H` : incluye archivos ocultos

## Links
- More information: https://docs.immich.app/features/command-line-interface/

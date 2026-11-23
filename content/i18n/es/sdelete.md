# sdelete

Elimina de forma segura archivos/directorios del disco, o limpia el espacio libre en un volumen/disco físico.

## Usage
- `sdelete -p 3 ruta\al\archivo1 ruta\al\archivo2 ...` : elimina archivos con 3 pasadas
- `sdelete -s ruta\al\directorio1 ruta\al\directorio2 ...` : elimina carpetas y sus subdirectorios con 1 pasada (por defecto)
- `sdelete -p 3 D:` : limpia el espacio libre del volumen D: con 3 pasadas
- `sdelete -z 2` : limpia el espacio libre con ceros z del disco físico 2, que no debe contener volúmenes a limpiar

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/sdelete

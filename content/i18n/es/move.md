# move

Mueve o renombra archivos y directorios. En PowerShell, este comando es un alias de `Move-Item`. Esta documentación se basa en la versión de `move` del Símbolo del sistema (`cmd`).

## Usage
- `tldr move-item` : muestra la documentación del comando equivalente de PowerShell
- `move ruta\al\origen ruta\al\destino` : renombra un archivo o directorio cuando el destino no es un directorio existente
- `move ruta\al\origen ruta\al\directorio_existente` : mueve un archivo o directorio a un directorio existente
- `move C:\ruta\al\origen D:\ruta\al\destino` : mueve un archivo o directorio entre unidades
- `move /Y ruta\al\origen ruta\al\directorio_existente` : no solicita confirmación antes de sobrescribir archivos existentes
- `move /-Y ruta\al\origen ruta\al\directorio_existente` : solicita confirmación antes de sobrescribir archivos existentes, independientemente de los permisos de archivo

# move-item

Mueve o renombra archivos, directorios, claves del registro y otros elementos de datos de PowerShell. Este comando solo se puede ejecutar a través de PowerShell.

## Usage
- `Move-Item ruta\al\origen ruta\al\destino` : renombra un archivo o directorio cuando el destino no es un directorio existente
- `Move-Item ruta\al\origen ruta\al\directorio_existente` : mueve un archivo o directorio a un directorio existente
- `Move-Item -LiteralPath "ruta\al\origen" ruta\al\archivo_o_directorio` : renombra o mueve archivo(s) con un nombre específico (no trata caracteres especiales dentro de cadenas)
- `Move-Item ruta\al\origen1 , ruta\al\origen2 ... ruta\al\directorio_existente` : mueve múltiples archivos a un directorio existente, manteniendo los nombres de archivo sin cambios
- `Move-Item ruta\a\la\clave_origen1 , ruta\a\la\clave_origen2 ... ruta\a\la\clave_nueva_o_existente` : mueve o renombra clave(s) del registro
- `mv -Force ruta\al\origen ruta\al\destino` : no solicita confirmación antes de sobrescribir archivos o claves del registro existentes
- `mv -Confirm ruta\al\origen ruta\al\destino` : solicita confirmación antes de sobrescribir archivos existentes, independientemente de los permisos de archivo
- `mv -WhatIf ruta\al\origen ruta\al\destino` : mueve archivos en modo de simulación, mostrando archivos y directorios que podrían ser movidos sin ejecutarlos

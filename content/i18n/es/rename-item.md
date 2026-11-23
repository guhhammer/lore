# rename-item

Comando de PowerShell para cambiar el nombre de un elemento. Nota: Tanto `ren` como `rni` pueden utilizarse como alias de `Rename-Item`.

## Usage
- `Rename-Item -Path "ruta\al\archivo" -NewName "nuevo_nombre_archivo"` : cambia el nombre de un archivo
- `Rename-Item -Path "ruta\al\directorio" -NewName "nuevo_nombre_directorio"` : cambia el nombre de un directorio
- `Rename-Item -Path "ruta\al\archivo" -NewName "ruta\al\nuevo_nombre_archivo"` : cambia el nombre y mueve un archivo
- `Rename-Item -Path "ruta\al\archivo" -NewName "nuevo_nombre_archivo" -Force` : cambia el nombre de un archivo de forma forzada
- `Rename-Item -Path "ruta\al\archivo" -NewName "nuevo_nombre_archivo" -Confirm` : solicita confirmación antes de cambiar el nombre de un archivo

# invoke-item

Abre archivos en sus respectivos programas predeterminados. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Invoke-Item -Path ruta\al\archivo` : abre un archivo en su programa predeterminado
- `Invoke-Item -Path ruta\al\directorio\*` : abre todos los archivos dentro de un directorio
- `Invoke-Item -Path ruta\al\directorio\*.png` : abre todos los PNG dentro de un directorio
- `Invoke-Item -Path ruta\al\directorio\* -Include *palabra_clave*` : abre todos los archivos dentro de un directorio que contengan una palabra clave específica
- `Invoke-Item -Path ruta\al\directorio\* -Exclude *palabra_clave*` : abre todos los archivos dentro de un directorio excepto aquellos que contengan una palabra clave específica
- `Invoke-Item -Path ruta\al\directorio\* -WhatIf` : realiza una simulación para determinar qué archivos se abrirán dentro de un directorio a través de `Invoke-Item`

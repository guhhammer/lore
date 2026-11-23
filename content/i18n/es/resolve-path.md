# resolve-path

Resuelve los caracteres comodín en una ruta y muestra el contenido de la ruta. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Resolve-Path ~` : resuelve la ruta de la carpeta de inicio
- `Resolve-Path -Path "\\nombre_del_host\ruta\al\archivo"` : resuelve una ruta UNC
- `Resolve-Path -Path ruta\al\archivo_o_directorio -Relative` : obtiene rutas relativas

# select-string

Busca texto en cadenas y archivos en PowerShell. Nota: Este comando solo puede usarse a través de PowerShell. Puedes usar `Select-String` de manera similar a `grep` en UNIX o `findstr.exe` en Windows.

## Usage
- `Select-String -Path "ruta\al\archivo" -Pattern 'patrón_de_búsqueda'` : busca un patrón dentro de un archivo
- `Select-String -SimpleMatch "cadena_exacta" ruta\al\archivo` : busca una cadena exacta (desactiva expresiones regulares)
- `Select-String -Path "*.ext" -Pattern 'patrón_de_búsqueda'` : busca un patrón en todos los archivos `.ext` en el directorio actual
- `Select-String --Context 2,3 "patrón_de_búsqueda" ruta\al\archivo` : captura el número especificado de líneas antes y después de la línea que coincide con el patrón
- `Get-Content ruta\al\archivo | Select-String --NotMatch "patrón_de_búsqueda"` : busca en `stdin` líneas que no coincidan con un patrón

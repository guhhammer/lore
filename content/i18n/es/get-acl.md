# get-acl

Obtiene el descriptor de seguridad para un recurso, como un archivo o una clave del registro. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Get-Acl ruta\al\directorio` : muestra la ACL para un directorio específico
- `Get-Acl -Path HKLM:\System\SetDeControlesActual\Control | Format-List` : obtiene una ACL para una clave del registro

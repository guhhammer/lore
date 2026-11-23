# set-acl

Cambia el descriptor de seguridad de un elemento especificado, como un archivo o una clave de registro. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `$OriginAcl = Get-Acl -Path ruta\al\archivo; Set-Acl -Path ruta\al\archivo -AclObject $OriginAcl` : copia un descriptor de seguridad de un archivo a otro
- `Get-Acl -Path ruta\al\archivo | Set-Acl -Path ruta\al\archivo` : usa el operador pipeline para pasar un descriptor

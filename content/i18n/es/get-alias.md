# get-alias

Lista y obtiene alias de comandos en la sesión actual de PowerShell. Este comando solo se puede ejecutar en PowerShell.

## Usage
- `Get-Alias` : lista todos los alias en la sesión actual
- `Get-Alias alias_de_comando` : obtiene el nombre del comando asociado al alias
- `Get-Alias -Definition comando` : lista todos los alias asignados a un comando específico
- `Get-Alias abc* -Exclude *def` : lista alias que comienzan con `abc`, excluyendo aquellos que terminan en `def`

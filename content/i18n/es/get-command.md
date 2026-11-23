# get-command

Lista y obtiene los comandos disponibles en la sesión actual de PowerShell. Este comando solo se puede ejecutar a través de PowerShell.

## Usage
- `Get-Command` : lista todos los comandos de PowerShell disponibles (alias, cmdlets, funciones) en el computador actual
- `Get-Command -ListImported` : lista todos los comandos de PowerShell disponibles en la sesión actual
- `Get-Command -Type Alias|Cmdlet|Function` : lista solo los alias/cmdlets/funciones de PowerShell disponibles en el computador
- `Get-Command -Type Application` : lista solo programas o comandos disponibles en PATH en la sesión actual
- `Get-Command -Module módulo` : lista solo comandos de PowerShell por el nombre del módulo, por ejemplo, `Microsoft.PowerShell.Utility` para comandos relacionados con utilidades
- `Get-Command comando` : obtiene la información del comando (por ejemplo, número de versión o nombre del módulo) por su nombre

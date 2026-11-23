# get-help

Muestra información de ayuda y documentación para los comandos de PowerShell (alias, cmdlets y funciones). Este comando solo se puede ejecutar a través de PowerShell.

## Usage
- `Get-Help comando` : muestra información de ayuda general para un comando específico de PowerShell
- `Get-Help comando -Detailed` : muestra una documentación más detallada para un comando específico de PowerShell
- `Get-Help comando -Full` : muestra la documentación técnica completa para un comando específico de PowerShell
- `Get-Help comando -Parameter parámetro` : imprime solo la documentación para un parámetro específico del comando de PowerShell (usa `*` para mostrar todos los parámetros), si está disponible
- `Get-Help comando -Examples` : imprime solo los ejemplos del cmdlet, si están disponibles
- `Get-Help *` : lista todas las páginas de ayuda de cmdlet disponibles
- `Update-Help` : actualiza la base de conocimientos de ayuda y documentación actual usando `Update-Help`
- `Get-Help comando -Online` : ve una versión en línea de la documentación del comando de PowerShell en el navegador web predeterminado

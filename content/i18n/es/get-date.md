# get-date

Obtiene la fecha y hora actuales. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Get-Date` : muestra la fecha y hora actuales
- `Get-Date -Format "yyyy-MM-dd HH:mm:ss"` : muestra la fecha y hora actuales con un especificador de formato .NET
- `(Get-Date).ToUniversalTime()` : muestra la fecha y hora actuales en UTC y en formato ISO 8601
- `Get-Date -UnixTimeSeconds 1577836800` : convierte un timestamp de Unix

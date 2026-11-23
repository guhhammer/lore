# get-dedupproperties

Obtiene información sobre la desduplicación de datos. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Get-DedupProperties -DriveLetter 'C'` : obtiene información sobre la desduplicación de datos de la unidad
- `Get-DedupProperties -FileSystemLabel 'Etiqueta'` : obtiene información sobre la desduplicación de datos de la unidad utilizando la etiqueta de la unidad
- `Get-DedupProperties -InputObject $(Get-Volume -DriveLetter 'E')` : obtiene información sobre la desduplicación de datos de la unidad utilizando el objeto de entrada

## Links
- More information: https://learn.microsoft.com/powershell/module/storage/get-dedupproperties

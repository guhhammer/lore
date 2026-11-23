# where-object

Selecteert objecten uit een verzameling op basis van hun eigenschapswaarden. Opmerking: dit commando kan alleen gebruikt worden via PowerShell.

## Usage
- `Get-Alias | Where-Object -Property Name -eq naam` : filter aliassen op naam
- `Get-Service | Where-Object {$_.Status -eq "Stopped"}` : toon een lijst van alle services die momenteel zijn gestopt. De `$_` automatische variabele representeert ieder object dat wordt gestuurd naar de `Where-Object` cmdlet
- `Get-Module -ListAvailable | Where-Object { $_.Name -NotLike "Microsoft*" -And $_.Name -NotLike "PS*" }` : gebruik meerdere condities

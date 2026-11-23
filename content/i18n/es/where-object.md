# where-object

Selecciona objetos de una colección basándose en los valores de sus propiedades. Nota: Este comando solo puede utilizarse a través de PowerShell.

## Usage
- `Get-Alias | Where-Object -Propiedad Nombre -eq nombre` : filtra alias por su nombre
- `Get-Service | Where-Object {$_.Status -eq "Stopped"}` : lista todos los servicios que están actualmente detenidos. La variable automática `$_` representa cada objeto que se pasa al cmdlet `Where-Object`
- `Get-Module -ListAvailable | Where-Object { $_.Name -NotLike "Microsoft*" -And $_.Name -NotLike "PS*" }` : utiliza múltiples condiciones

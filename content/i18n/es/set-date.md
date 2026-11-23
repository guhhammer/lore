# set-date

Cambia la hora del sistema en el equipo a una hora que especifiques. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Set-Date -Date (Get-Date).AddDays(3)` : agrega tres días a la fecha del sistema
- `Set-Date -Adjust -0:10:0 -DisplayHint Time` : retrasa el reloj del sistema 10 minutos
- `$90mins = New-TimeSpan -Minutes 90; Set-Date -Adjust $90mins` : agrega 90 minutos al reloj del sistema

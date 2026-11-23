# out-string

Salida de objetos de entrada como una cadena. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Get-Alias | Out-String` : imprime información del host como cadena
- `Get-Alias | Out-String -Stream` : convierte cada objeto a una cadena en lugar de concatenar todos los objetos en una sola cadena
- `@{TestKey = ('x' * 200)} | Out-String -Width 250` : usa el parámetro `Width` (Ancho) para evitar la truncación

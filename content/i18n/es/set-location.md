# set-location

Muestra el directorio de trabajo actual o va a un directorio diferente. Nota: Este comando sólo se puede utilizar a través de PowerShell.

## Usage
- `Set-Location ruta\al\directorio` : ir al directorio especificado
- `Set-Location C:ruta\al\directorio` : ir a un directorio específico en una unidad diferente
- `Set-Location ruta\al\directorio -PassThru` : ir y muestra la ubicación del directorio especificado
- `Set-Location ..` : sube al directorio padre del directorio actual
- `Set-Location ~` : ir al directorio principal del usuario actual
- `Set-Location -|+` : regresa/va al directorio elegido anteriormente
- `Set-Location \` : ir a la raíz de la unidad actual

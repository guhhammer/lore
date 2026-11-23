# measure-object

Calcula las propiedades numéricas de los objetos, así como los caracteres, palabras y líneas en objetos de cadena, como archivos de texto. Nota: Este comando solo se puede usar a través de PowerShell.

## Usage
- `Get-ChildItem | Measure-Object` : cuenta los archivos y carpetas en un directorio
- `"Uno", "Dos", "Tres", "Cuatro" | Set-Content -Path "ruta\al\archivo"; Get-Content "ruta\al\archivo"; | Measure-Object -Character -Line -Word` : pasa la entrada a Measure-Object (los objetos que se pasan a `Measure-Object` están disponibles para el bloque de script que se pasa al parámetro Expression)

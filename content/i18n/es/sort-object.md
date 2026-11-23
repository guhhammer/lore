# sort-object

Ordena objetos por valores de propiedades. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Get-ChildItem | Sort-Object` : ordena el directorio actual por nombre
- `Get-ChildItem | Sort-Object -Descending` : ordena el directorio actual por nombre en orden descendente
- `"a", "b", "a" | Sort-Object -Unique` : ordena elementos eliminando duplicados
- `Get-ChildItem | Sort-Object -Property Length` : ordena el directorio actual por tamaño de archivo (longitud)
- `Get-Process | Sort-Object -Property WS` : ordena procesos por el uso más alto de memoria basado en su tamaño de conjunto de trabajo (WS)

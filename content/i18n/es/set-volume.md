# set-volume

Establece o cambia la etiqueta del sistema de archivos de un volumen existente. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Set-Volume -DriveLetter "D" -NewFileSystemLabel "VolumenDatos"` : cambia la etiqueta del sistema de archivos de un volumen identificado por la letra de unidad
- `Set-Volume -FileSystemLabel "EtiquetaVieja" -NewFileSystemLabel "EtiquetaNueva"` : cambia la etiqueta del sistema de archivos de un volumen identificado por la etiqueta del sistema
- `Set-Volume -InputObject $(Get-Volume -DriveLetter "E") -NewFileSystemLabel "Respaldo"` : modifica las propiedades de un volumen usando un objeto volumen
- `Set-Volume -DriveLetter "D" -DedupMode Backup` : especifica el modo de desduplicación de datos para el volumen

## Links
- More information: https://learn.microsoft.com/powershell/module/storage/set-volume

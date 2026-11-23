# set-clipboard

Comando de PowerShell para copiar contenido al portapapeles. Nota: Se puede utilizar `scb` como alias de `Set-Clipboard`.

## Usage
- `Set-Clipboard -Value "texto"` : copia texto al portapapeles
- `Set-Clipboard -Value @("texto 1", "texto 2", "texto 3")` : copia varios textos al portapapeles separados por una nueva línea
- `Set-Clipboard -Path "ruta\a\archivos_o_directorios"` : copia archivos o directorios al portapapeles
- `Set-Clipboard -Path "ruta\a\archivo1","ruta\a\archivo2","ruta\a\archivo3"` : copia varios archivos
- `Set-Clipboard ""` : limpia el portapapeles

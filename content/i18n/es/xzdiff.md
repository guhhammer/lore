# xzdiff

Ejecuta `diff` en archivos comprimidos con formatos de compresión `xz`, `lzma`, `gzip`, `bzip2`, `lzop` o `zstd`. Todas las opciones especificadas se pasan directamente a `diff`.

## Usage
- `xzdiff ruta/al/archivo1 ruta/al/archivo2` : compara dos archivos
- `xzdiff --side-by-side ruta/al/archivo1 ruta/al/archivo2` : compara dos archivos, mostrando las diferencias en paralelo
- `xzdiff --brief ruta/al/archivo1 ruta/al/archivo2` : compara dos archivos e informa solo de que difieren (sin detalles sobre qué es en lo que difiere)
- `xzdiff --report-identical-files ruta/al/archivo1 ruta/al/archivo2` : compara dos archivos e informa de cuándo los archivos son iguales
- `xzdiff --paginate ruta/al/archivo1 ruta/al/archivo2` : compara dos archivos utilizando resultados paginados

## Links
- More information: https://manned.org/xzdiff

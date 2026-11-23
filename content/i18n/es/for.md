# for

Ejecuta condicionalmente un comando varias veces.

## Usage
- `for %variable in (elemento_a elemento_b elemento_c) do (echo Se ejecuta el bucle)` : ejecuta los comandos dados para el conjunto especificado
- `for /l %variable in (desde, paso, hasta) do (echo Se ejecuta el bucle)` : itera sobre un rango dado de números
- `for %variable in (ruta\al\archivo1.ext ruta\al\archivo2.ext ...) do (echo Se ejecuta el bucle)` : itera sobre una lista dada de archivos
- `for /d %variable in (ruta\al\directorio1.ext ruta\al\directorio2.ext ...) do (echo Se ejecuta el bucle)` : itera sobre una lista dada de directorios
- `for /d %variable in (*) do (if exist %variable echo Se ejecuta el bucle)` : realiza un comando dado en cada directorio

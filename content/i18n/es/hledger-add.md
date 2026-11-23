# hledger add

Registra nuevas transacciones con mensajes interactivos en la consola.

## Usage
- `hledger add` : registra nuevas transacciones, guardándolas al archivo de diario por defecto
- `hledger add -f ruta/a/2024.journal -f ruta/a/2023.journal` : añade transacciones a `2024.journal`, pero también carga `2023.journal` para su completado
- `hledger add today 'best buy' gastos:material de oficina '$20'` : provee respuestas a las primeras 4 preguntas
- `hledger add -h` : muestra la documentación y opciones de `add` usando `$PAGER`
- `hledger help add` : muestra la documentación de `add` usando `info` o `man` de estar disponibles

## Links
- More information: https://hledger.org/hledger.html#add

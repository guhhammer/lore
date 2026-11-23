# inotifywait

Espera a que se produzcan cambios en los archivos.

## Usage
- `inotifywait ruta/al/archivo` : vigila un archivo específico en busca de eventos, saliendo tras el primero
- `inotifywait -m ruta/al/archivo` : vigila continuamente un archivo específico en busca de eventos sin salir
- `inotifywait -m -r ruta/al/directorio` : vigila un directorio de forma recursiva en busca de eventos
- `inotifywait -m -r --exclude "regex" ruta/al/directorio` : vigila un directorio en busca de cambios, excluyendo los archivos cuyos nombres coincidan con una `expresión regular`
- `inotifywait -m -t 30 ruta/al/archivo` : vigila un archivo en busca de cambios, cerrándose cuando no se produzca ningún evento durante 30 segundos
- `inotifywait -e modify ruta/al/archivo` : vigila un archivo solo en busca de eventos de modificación
- `inotifywait -q ruta/al/archivo` : vigila un archivo mostrando solo los eventos, sin mensajes de estado
- `inotifywait -e access ruta/al/archivo && comando` : ejecuta un comando cuando se accede a un archivo

## Links
- More information: https://manned.org/inotifywait

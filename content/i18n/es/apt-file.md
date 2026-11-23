# apt-file

Busca archivos en paquetes `apt`, incluyendo los que aún no han sido instalados.

## Usage
- `sudo apt update` : actualiza los metadatos de la base de datos
- `apt-file find ruta/al/archivo` : busca paquetes que contengan el archivo o ruta especificada
- `apt-file list paquete` : muestra el contenido del paquete especificado
- `apt-file find -x expresión_regular` : busca paquetes que coincidan con la `expresión_regular`

## Links
- More information: https://manned.org/apt-file

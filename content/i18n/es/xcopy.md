# xcopy

Copia archivos y árboles de directorios.

## Usage
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino` : copia el o los archivos a la ubicación de destino especificada
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /p` : lista los archivos que se copiarán antes de copiarlos
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /t` : copia solo la estructura del directorio, excluyendo archivos
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /e` : incluye directorios vacíos al copiar
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /o` : mantiene el ACL de origen en el destino
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /z` : permite reanudar cuando se pierde la conexión de red
- `xcopy ruta\al\archivo_o_directorio ruta\al\directorio_de_destino /y` : desactiva el aviso cuando el archivo existe en el destino
- `xcopy /?` : muestra la ayuda

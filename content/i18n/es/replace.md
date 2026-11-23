# replace

Reemplaza archivos.

## Usage
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino` : reemplaza el archivo de destino con el del directorio origen
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /a` : agrega archivos al directorio destino en lugar de reemplazar archivos existentes
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /p` : copia múltiples archivos de forma interactiva, con una solicitud antes de reemplazar o agregar un archivo destino
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /r` : reemplaza incluso archivos de solo lectura
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /w` : espera a que insertes un disco antes de reemplazar archivos (originalmente para permitir insertar un disquete)
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /s` : reemplaza todos los archivos en subdirectorios del destino
- `replace ruta\al\archivo_o_directorio ruta\al\directorio_destino /u` : reemplaza solo los archivos en el directorio destino que sean más antiguos que los archivos en el directorio origen
- `replace /?` : muestra la ayuda

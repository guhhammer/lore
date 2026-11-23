# objcopy

Copia el contenido de un archivo de objetos a otro archivo.

## Usage
- `objcopy ruta/al/archivo_de_origen ruta/al/archivo_de_destino` : copia datos a otro archivo
- `objcopy --input-target=formato_de_entrada --output-target formato_de_salida ruta/al/archivo_de_origen ruta/al/archivo_de_destino` : traduce ficheros de un formato a otro
- `objcopy --strip-all ruta/al/archivo_de_origen ruta/al/archivo_de_destino` : elimina toda la información de símbolos del archivo
- `objcopy --strip-debug ruta/al/archivo_de_origen ruta/al/archivo_de_destino` : elimina la información de depuración del archivo
- `objcopy --only-section section ruta/al/archivo_de_origen ruta/al/archivo_de_destino` : copia una sección específica del archivo de origen al archivo de destino

## Links
- More information: https://manned.org/objcopy

# protoc

Analiza los archivos `.proto` de Google Protobuf y genera la salida en el idioma especificado.

## Usage
- `protoc --python_out=ruta/al/directorio_salida archivo_entrada.proto` : genera código Python a partir de un archivo `.proto`
- `protoc --java_out=ruta/al/directorio_salida --proto_path=ruta/a/importación_ruta_de_busqueda archivo_entrada.proto` : genera código Java a partir de un archivo `.proto` que importa otros archivos `.proto`
- `protoc --csharp_out=ruta/a/c#_directorio_salida --js_out=ruta/a/js_directorio_salida archivo_entrada.proto` : genera código para múltiples lenguajes
- `protoc <mensaje.txt --encode=TypeName archivo_de_entrada.proto` : codifica un mensaje en formato texto en un mensaje de protocolo a partir de un archivo `.proto`
- `protoc <mensaje.bin --decode=TypeName archivo_entrada.proto` : decodifica un mensaje de protocolo en formato de texto a partir de un archivo `.proto`
- `protoc <mensaje.bin --decode_raw` : decodifica un mensaje de protocolo en pares de etiquetas/valores sin procesar

## Links
- More information: https://manned.org/protoc

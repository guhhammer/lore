# certutil

Una herramienta para gestionar y configurar información de certificados.

## Usage
- `certutil nombre_de_archivo` : exporta la información de configuración o archivos
- `certutil -encodehex ruta\al\archivo_entrada ruta\al\archivo_salida` : encripta un archivo en hexadecimal
- `certutil -encode ruta\al\archivo_entrada ruta\al\archivo_salida` : encripta un archivo a Base64
- `certutil -decode ruta\al\archivo_entrada ruta\al\archivo_salida` : decodifica un archivo codificado en Base64
- `certutil -hashfile ruta\al\archivo_entrada md2|md4|md5|sha1|sha256|sha384|sha512` : genera y muestra un hash criptográfico sobre un archivo

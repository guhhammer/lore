# pdfdetach

Lista o extrae archivos adjuntos (archivos embebidos) de un archivo PDF.

## Usage
- `pdfdetach list -enc UTF-8 ruta/al/archivo.pdf` : lista todos los archivos adjuntos en un archivo con una codificación de texto específica
- `pdfdetach -save número ruta/a/la/entrada.pdf` : guarda un archivo embebido especificado por su número
- `pdfdetach -savefile nombre ruta/al/archivo.pdf` : guarda un archivo embebido especificando su nombre
- `pdfdetach -save número -o ruta/al/resultado ruta/a/la/entrada.pdf` : guarda el archivo embebido con un nombre de archivo de salida personalizado
- `pdfdetach -save número -opw|-upw contraseña ruta/a/la/entrada.pdf` : guarda el adjunto desde un archivo protegido por contraseña del propietario/usuario

## Links
- More information: https://manned.org/pdfdetach

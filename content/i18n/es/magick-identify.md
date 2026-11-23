# magick identify

Describe el formato y las características de los archivos de imagen.

## Usage
- `magick identify ruta/a/la/imagen` : describe el formato y las características básicas de una imagen
- `magick identify -verbose ruta/a/la/imagen` : describe el formato y las características de una imagen detalladamente
- `magick identify -format "%f,%w,%h\n" *.jpg >ruta/al/archivo.csv` : recopila las dimensiones de todos los archivos JPEG en el directorio actual y los guarda en un archivo CSV

## Links
- More information: https://imagemagick.org/script/identify.php

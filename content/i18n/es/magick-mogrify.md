# magick mogrify

Realiza operaciones en múltiples imágenes, como redimensionar, recortar, voltear y añadir efectos. Los cambios se aplican directamente al archivo original.

## Usage
- `magick mogrify -resize 50% *.jpg` : redimensiona todas las imágenes JPEG en el directorio al 50% de su tamaño inicial
- `magick mogrify -resize 800x600 DSC*` : redimensiona todas las imágenes comenzando con `DSC` a 800x600
- `magick mogrify -format jpg *.png` : convierte todos los PNG a JPEG
- `magick mogrify -modulate 100,50 *` : redimensiona todas las imágenes JPEG en el directorio al 50% de su tamaño inicial
- `magick mogrify -modulate 200 *` : dobla el brillo de todos los archivos de imagen en el directorio actual
- `magick mogrify -layers 'optimize' -fuzz 7% *.gif` : reduce tamaños de archivos de todas las imágenes GIF en el directorio actual reduciendo la calidad
- `magick mogrify -help` : muestra la ayuda

## Links
- More information: https://imagemagick.org/script/mogrify.php

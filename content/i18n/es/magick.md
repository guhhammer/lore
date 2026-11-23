# magick

Crea, edita, compone o convierte entre formatos de imagen. Esta herramienta reemplaza a `convert` en ImageMagick 7+. Usa `magick convert` para utilizar la herramienta antigua en versiones 7+. Algunos subcomandos, como `mogrify`, tienen su propia documentación de uso.

## Usage
- `magick ruta/a/imagen_entrada.png ruta/a/imagen_salida.jpg` : convierte entre formatos de imagen
- `magick ruta/a/imagen_entrada.jpg -resize 100x100 ruta/a/imagen_salida.jpg` : redimensiona una imagen, creando una nueva copia
- `magick *.jpg ruta/a/imagenes.gif` : crea un GIF a partir de todas las imágenes JPEG en el directorio actual
- `magick -size 640x480 pattern:checkerboard ruta/a/tablero.png` : crea un patrón de tablero de ajedrez
- `magick *.jpg -adjoin ruta/al/archivo.pdf` : crea un archivo PDF a partir de todas las imágenes JPEG en el directorio actual

## Links
- More information: https://imagemagick.org/script/magick.php

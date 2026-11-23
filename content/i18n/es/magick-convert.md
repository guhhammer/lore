# magick convert

Convierte entre formatos de imagen, escala, une y crea imágenes. Nota: esta herramienta (anteriormente `convert`) ha sido reemplazada por `magick` en ImageMagick 7+.

## Usage
- `magick convert ruta/a/imagen_entrada.jpg ruta/a/imagen_salida.png` : convierte una imagen de JPEG a PNG
- `magick convert ruta/a/imagen_entrada.png -resize 50% ruta/a/imagen_salida.png` : escala una imagen al 50% de su tamaño original
- `magick convert ruta/a/imagen_entrada.png -resize 640x480 ruta/a/imagen_salida.png` : escala una imagen manteniendo la relación de aspecto original a un tamaño máximo de 640x480
- `magick convert ruta/a/imagen_entrada.png -define jpeg:extent=512kb ruta/a/imagen_salida.jpg` : escala una imagen para tener un tamaño de archivo específico
- `magick convert ruta/a/imagen1.png ruta/a/imagen2.png ... -append|+append ruta/a/imagen_salida.png` : anexa imágenes verticalmente/horizontalmente
- `magick convert ruta/a/imagen1.png ruta/a/imagen2.png ... -delay 10 ruta/a/animacion.gif` : crea un GIF a partir de una serie de imágenes con un retraso de 100ms entre ellas
- `magick convert -size 800x600 "xc:#ff0000" ruta/a/imagen.png` : crea una imagen con solo un fondo rojo sólido
- `magick convert ruta/a/imagen1.png ruta/a/imagen2.png ... ruta/a/favicon.ico` : crea un favicon a partir de varias imágenes de diferentes tamaños

## Links
- More information: https://imagemagick.org/script/convert.php

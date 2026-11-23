# magick convert

Outil de conversion d'image d'ImageMagick.

## Usage
- `magick convert image.jpg image.png` : convertir une image JPEG en PNG
- `magick convert image.png -resize 50% image2.png` : redimensionner une image à 50% de ses dimensions d'origine
- `magick convert image.png -resize 640x480 image2.png` : redimensionner une image en conservant son ratio hauteur/largeur initial pour une taille maximum de 640x480
- `magick convert image1.png image2.png image3.png +append image123.png` : coller plusieurs images horizontallement
- `magick convert image1.png image2.png image3.png -append image123.png` : coller plusieurs images verticalement
- `magick convert image1.png image2.png image3.png -delay 100 animation.gif` : créer un gif à partir d'une série d'images avec un délai de 100ms entre chaque
- `magick convert -size 800x600 "xc:#ff0000" image.png` : créer une image avec un simple arrière-plan uni

## Links
- More information: https://imagemagick.org/script/convert.php

# magick convert

Strumento della suite immagineMagick per la conversione di immagini.

## Usage
- `magick convert immagine.jpg immagine.png` : converti un'immagine da JPEG a PNG
- `magick convert immagine.png -resize 50% immagine2.png` : scala un'immagine al 50% delle sue dimensioni originali
- `magick convert immagine.png -resize 640x480 immagine2.png` : scala un'immagine ad una dimensione massima di 640x480 mantenendo le proporzioni originali
- `magick convert immagine1.png immagine2.png immagine3.png +append immagine123.png` : concatena più immagini orizzontalmente
- `magick convert immagine1.png immagine2.png immagine3.png -delay 100 animazione.gif` : crea una GIF da una serie di immagini con un intervallo di 100ms tra ogni immagine
- `magick convert -size 800x600 "xc:#ff0000" immagine.png` : crea un'immagine a tinta unita di un determinato colore

## Links
- More information: https://imagemagick.org/script/convert.php

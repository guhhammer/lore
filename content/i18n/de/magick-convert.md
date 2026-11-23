# magick convert

ImageMagick Bildkonvertierungswerkzeug.

## Usage
- `magick convert pfad/zu/bild.jpg pfad/zu/bild.png` : konvertiere ein Bild von JPEG nach PNG
- `magick convert pfad/zu/bild.png -resize 50% pfad/zu/bild2.png` : skaliere ein Bild auf 50% seiner Originalgröße
- `magick convert pfad/zu/bild.png -resize 640x480 pfad/zu/bild2.png` : skaliere ein Bild unter Beibehaltung des ursprünglichen Seitenverhältnisses auf eine maximale Größe von 640x480
- `magick convert pfad/zu/bild1.png pfad/zu/bild2.png ... -append|+append pfad/zu/bild.png` : hänge Bilder vertikal/horizontal aneinander
- `magick convert pfad/zu/bild1.png pfad/zu/bild2.png ... -delay 10 pfad/zu/animation.gif` : erstelle ein animiertes GIF aus einer Serie von Bildern mit einer Verzögerung von 100 ms zwischen den Bildern
- `magick convert -size 800x600 "xc:#ff0000" pfad/zu/bild.png` : erstelle ein Bild mit nichts als einem festen Hintergrund
- `magick convert pfad/zu/bild1.png pfad/zu/bild2.png ... pfad/zu/bild.ico` : erstelle ein Favicon aus mehreren Bildern verschiedener Größe

## Links
- More information: https://imagemagick.org/script/convert.php

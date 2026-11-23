# magick import

Captura parte o toda la pantalla de un servidor X y guarda la imagen en un archivo.

## Usage
- `magick import -window root ruta/a/salida.ps` : captura toda la pantalla del servidor X en un archivo PostScript
- `magick import -window root -display servidor_remoto:pantalla.display ruta/a/salida.png` : captura el contenido de la pantalla de un servidor X remoto en una imagen PNG
- `magick import -window id_ventana ruta/a/salida.jpg` : captura una ventana específica dada su ID mostrada por `xwininfo` en una imagen JPEG

## Links
- More information: https://imagemagick.org/script/import.php

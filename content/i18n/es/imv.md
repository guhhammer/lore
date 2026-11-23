# imv

Visor de imágenes de línea de comando para Wayland y X11 dirigido a gestores de ventanas en mosaico. Maneja múltiples formatos incluyendo Photoshop (PSD).

## Usage
- `imv ruta/a/la/imagen1.ext ruta/a/la/imagen2.ext ...` : muestra múltiples imágenes
- `imv -f ruta/a/la/imagen.ext` : vista en modo de pantalla completa
- `imv -r --slideshow ruta/al/directorio` : muestra imágenes de un directorio recursivamente
- `find . -type f -name "*.svg" | imv` : abre múltiples imágenes vía `stdin`
- `imv -t 10 ruta/al/directorio` : hace una presentación desde un directorio que muestra cada imagen durante 10 segundos
- `curl -Osw '%{filename_effective}\n' 'http://www.example.com/[1-10].jpg' | imv` : muestra múltiples imágenes de la web

## Links
- More information: https://sr.ht/~exec64/imv/

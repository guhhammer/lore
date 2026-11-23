# feh

Utilidad ligera de visualización de imágenes.

## Usage
- `feh ruta/a/imagen` : muestra imágenes localmente o usando una URL
- `feh --recursive ruta/al/directorio` : muestra imágenes recursivamente
- `feh --borderless ruta/a/imagen` : muestra imágenes sin bordes
- `feh --cycle-once ruta/a/imagen` : cierra después de la última imagen
- `feh --slideshow-delay secundos ruta/a/imagen` : agrega una demora al ciclo de la presentación
- `feh --bg-center|fill|max|scale|tile ruta/a/imagen` : cambia el fondo de pantalla (centrado, llenar, maximizado, ampliado o amontonado)
- `feh --montage --thumb-height 150 --thumb-width 150 --index-info "%nn%wx%h" --output ruta/a/nueva_imagen` : crea un montaje de todas las imágenes en un directorio. Produce una nueva imagen

## Links
- More information: https://man.finalrewind.org/1/feh/

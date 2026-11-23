# sips

Sistema de procesamiento de imágenes Apple Scriptable. Imágenes Raster/Query y Perfiles ICC ColorSync.

## Usage
- `sips --out ruta/al/directorio_salida` : especifica un directorio de salida para que los originales no se modifiquen
- `sips --resampleHeightWidth 1920 300 archivo_imagen.ext` : remuestrea la imagen al tamaño especificado, la relación de aspecto de la imagen puede verse alterada
- `sips --resampleHeightWidthMax 1920 300 archivo_imagen.ext` : remuestrea la imagen para que la altura y la anchura no superen el tamaño especificado (fíjate en la Z mayúscula)
- `sips --resampleWidth 960 ruta/al/imágenes` : remuestrea todas las imágenes de un directorio para que se ajusten a una anchura de 960px (respetando la relación de aspecto)
- `sips --matchTo "/System/Library/ColorSync/Profiles/Generic RGB Profile.icc" ruta/al/imagen.ext --out ruta/al/directorio_salida` : convierte una imagen de CMYK a RGB
- `sips --deleteProperty profile --deleteColorManagementProperties ruta/al/archivo_de_imagen.ext` : elimina el perfil ICC ColorSync de una imagen

## Links
- More information: https://keith.github.io/xcode-man-pages/sips.1.html

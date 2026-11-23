# hdiutil

Utilidad para crear y gestionar imágenes de disco.

## Usage
- `hdiutil attach ruta/al/archivo_de_imagen` : monta una imagen
- `hdiutil detach /Volumes/archivo_de_volumen` : desmonta una imagen
- `hdiutil info` : muestra las imágenes montadas
- `hdiutil makehybrid -o ruta/al/archivo_de_salida ruta/al/directorio` : crea una imagen ISO a partir del contenido de un directorio

## Links
- More information: https://keith.github.io/xcode-man-pages/hdiutil.1.html

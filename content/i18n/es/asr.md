# asr

Restaura (copia) una imagen de disco en un volumen. El nombre del comando significa Restauración de Software de Apple.

## Usage
- `sudo asr restore --source nombre_de_imagen.dmg --target ruta/al/volumen` : restaura una imagen de disco en un volumen
- `sudo asr restore --source nombre_de_imagen.dmg --target ruta/al/volumen --erase` : borra el volumen deseado antes de restaurar
- `sudo asr restore --source nombre_de_imagen.dmg --target ruta/al/volumen --noverify` : omite la verificación después de restaurar
- `sudo asr restore --source ruta/al/volumen --target ruta/al/volumen_clonado` : clona volúmenes sin el uso de una imagen de disco intermedia

## Links
- More information: https://keith.github.io/xcode-man-pages/asr.8.html

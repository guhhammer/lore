# pamcut

Corta una región rectangular de una imagen Netpbm.

## Usage
- `pamcut -cropl valor -cropr valor -cropt valor -cropb valor ruta/a/la/imagen.ppm >ruta/al/resultado.ppm` : descarta la cantidad de columnas/filas especificadas a cada lado de la imagen
- `pamcut -l valor -ri valor ruta/a/la/imagen.ppm >ruta/al/resultado.ppm` : mantiene solo las columnas entre las columnas especificadas (inclusivamente)
- `pamcut -t valor -b valor -pad ruta/a/la/imagen.ppm >ruta/al/resultado.ppm` : llena áreas perdidas con píxeles negros si el rectángulo especificado no se encuentra completamente dentro de la imagen de entrada

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcut.html

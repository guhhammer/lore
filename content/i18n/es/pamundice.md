# pamundice

Combina una cuadrícula de imágenes Netpbm en una sola.

## Usage
- `pamundice nombrearchivo_%1d_%1a.ppm -a grid_width -d grid_height >ruta/a/salida.ppm` : combina las imágenes cuyos nombres coincidan con la expresión de nombre de archivo al estilo `printf`. Supongamos una cuadrícula con un tamaño específico
- `pamundice nombrearchivo_%1d_%1a.ppm -a valor_x -d valor_y -ho valor -vo valor >ruta/a/salida.ppm` : supongamos que los mosaicos se solapan horizontal y verticalmente en la cantidad especificada
- `pamundice -l ruta/a/archivo.txt -a valor_x -d valor_y >ruta/a/salida.ppm` : especifica las imágenes que se van a combinar mediante un archivo de texto que contenga un nombre de archivo por línea

## Links
- More information: https://netpbm.sourceforge.net/doc/pamundice.html

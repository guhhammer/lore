# pcdovtoppm

Crea una imagen índice para un CD de fotos basándose en su archivo de resumen.

## Usage
- `pcdovtoppm ruta/al/archivo.pcd >ruta/al/archivo.ppm` : crea una imagen índice PPM a partir de un archivo de vista general PCD
- `pcdovtoppm -m anchura -s tamaño ruta/al/archivo.pcd >ruta/al/archivo.ppm` : especifica la anchura máxima de la imagen de salida y el tamaño máximo de cada una de las imágenes contenidas en la salida
- `pcdovtoppm -a n_imágenes -c n_colores ruta/al/archivo.pcd >ruta/al/archivo_salida.ppm` : especifica el número máximo de imágenes y el número máximo de colores
- `pcdovtoppm -f fuente -w ruta/al/archivo.pcd >ruta/al/archivo.ppm` : utiliza la fuente especificada para las anotaciones y pinta el fondo blanco

## Links
- More information: https://netpbm.sourceforge.net/doc/pcdovtoppm.html

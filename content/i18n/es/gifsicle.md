# gifsicle

Crea, edita, manipula y obtiene información sobre archivos GIF.

## Usage
- `gifsicle ruta/a/archivo_entrada.gif -O3 -o ruta/a/archivo_salida.gif` : optimiza un GIF como un nuevo archivo
- `gifsicle -b ruta/a/archivo_entrada.gif -U` : utiliza el modo por lotes (modifica cada archivo dado su lugar) y desoptimiza un archivo GIF
- `gifsicle ruta/a/archivo_entrada.gif '#0' >ruta/a/primer_fotograma.gif` : extrae un fotograma de un archivo GIF
- `gifsicle *.gif -d 10 -l >ruta/a/archivo_salida.gif` : crea una animación GIF a partir de GIF seleccionados
- `gifsicle -b ruta/a/archivo_entrada.gif -O3 --lossy=100 -k 16 -f` : reduce el tamaño del archivo utilizando compresión con pérdida
- `gifsicle -b ruta/a/archivo_entrada.gif --delete '#0-9' '#20-'` : elimina los primeros 10 fotogramas y todos los fotogramas posteriores al fotograma 20 de un GIF
- `gifsicle -b --crop starting_x,starting_y+rect_widthxrect_height --scale 0.25 --flip-horizontal --rotate-90|180|270 ruta/a/archivo_entrada.gif` : modifica todos los fotogramas recortándolos en un rectángulo, cambiando su escala, volteándolos y rotándolos

## Links
- More information: https://www.lcdf.org/gifsicle/

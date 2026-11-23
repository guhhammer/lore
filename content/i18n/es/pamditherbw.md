# pamditherbw

Aplica dithering a una imagen en escala de grises, es decir, la convierte en un patrón de píxeles blancos y negros que parecen iguales a la escala de grises original.

## Usage
- `pamditherbw ruta/a/la/imagen.pgm >ruta/al/archivo.pgm` : lee una imagen PGM, aplica la separación y la guarda en un archivo
- `pamditherbw -floyd|fs|atkinson|threshold|hilbert|... ruta/a/la/imagen.pgm >ruta/al/archivo.pgm` : utiliza el método de cuantización especificado
- `pamditherbw -a -r 1337 ruta/a/la/imagen.pgm >ruta/al/archivo.pgm` : utiliza el método de cuantización de atkinson y la semilla especificada para un generador de número pseudo-aleatorio
- `pamditherbw -fs|atkinson|thresholding -va 0.3 ruta/a/la/imagen.pgm >ruta/al/archivo.pgm` : especifica el valor de umbralización (thresholding) para los métodos de cuantización que realizan algún tipo de umbralización

## Links
- More information: https://netpbm.sourceforge.net/doc/pamditherbw.html

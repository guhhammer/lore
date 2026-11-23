# pnmcolormap

Crea mapa de colores cuantizado para una imagen PNM.

## Usage
- `pnmcolormap n_colores ruta/a/la/entrada.pnm >ruta/al/resultado.ppm` : genera una imagen usando sólo 'n_colores' o menos colores lo más cerca posible de la imagen de entrada
- `pnmcolormap -splits n_colores ruta/a/la/entrada.pnm >ruta/al/resultado.ppm` : utiliza la estrategia de splitspread para determinar los colores de salida, posiblemente produciendo un mejor resultado para imágenes con detalles pequeños
- `pnmcolormap -so ruta/a/la/entrada.pnm >ruta/al/resultado.ppm` : ordena el mapa de colores resultante, que es útil para comparar los mapas de colores

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmcolormap.html

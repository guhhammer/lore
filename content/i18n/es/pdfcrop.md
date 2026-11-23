# pdfcrop

Detecta y elimina márgenes en cada página de un archivo PDF.

## Usage
- `pdfcrop ruta/al/archivo_inicial.pdf ruta/al/archivo_resultado.pdf` : detecta y elimina automáticamente el margen de cada página en un archivo PDF
- `pdfcrop ruta/al/archivo_inicial.pdf --margins 'izquierda arriba derecha abajo' ruta/al/archivo_resultado.pdf` : establece los márgenes de cada página a un valor específico
- `pdfcrop ruta/al/archivo_de_entrada.pdf --margins 300 ruta/al/archivo_resultado.pdf` : establece los márgenes de cada página a un valor específico, utilizando el mismo valor para izquierda, arriba, derecha y abajo
- `pdfcrop ruta/al/archivo_de_entrada.pdf --bbox 'izquierda arriba derecha abajo' ruta/al/archivo_resultado.pdf` : utiliza una caja de encuadre definida por el usuario para cortar en lugar de detectarla automáticamente
- `pdfcrop ruta/al/archivo_de_entrada.pdf --bbox-odd 'izquierda arriba derecha abajo' --bbox-even 'izquierda arriba derecha abajo' ruta/al/archivo_resultado.pdf` : utiliza diferentes cajas de encuadre definidas por el usuario para páginas pares e impares
- `pdfcrop ruta/al/archivo_de_entrada.pdf --resolution 72 ruta/al/archivo_resultado.pdf` : detecta márgenes automáticamente utilizando una resolución menor para mejorar el rendimiento

## Links
- More information: https://github.com/ho-tex/pdfcrop

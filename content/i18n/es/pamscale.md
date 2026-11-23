# pamscale

Escala una imagen Netpbm.

## Usage
- `pamscale -wid ancho -h alto ruta/a/entrada.pam >ruta/a/salida.pam` : escala una imagen de modo que el resultado tenga las dimensiones especificadas
- `pamscale -wid ancho ruta/a/entrada.pam >ruta/a/salida.pam` : escala una imagen de modo que el resultado tenga la anchura especificada, manteniendo la relación de aspecto
- `pamscale -xsc x_factor -ysc y_factor ruta/a/entrada.pam >ruta/a/salida.pam` : escala una imagen de modo que su anchura y altura se modifiquen según los factores especificados
- `pamscale -xyfit bbox_width bbox_height ruta/a/entrada.pam >ruta/a/salida.pam` : escala una imagen de modo que quepa en el cuadro delimitador especificado conservando su relación de aspecto
- `pamscale -xyfill ancho_cuadro box_height ruta/a/entrada.pam >ruta/a/salida.pam` : escala una imagen de modo que llene completamente el cuadro especificado conservando su relación de aspecto

## Links
- More information: https://netpbm.sourceforge.net/doc/pamscale.html

# pamslice

Extrae una línea de valores de una imagen PAM.

## Usage
- `pamslice -r n ruta/a/imagen.pam` : imprime los valores de los píxeles de la n-ésima fila en una tabla
- `pamslice -c n ruta/a/imagen.pam` : imprime los valores de los píxeles de la n-ésima columna de una tabla
- `pamslice -r n -plane m ruta/a/imagen.pam` : considera solo el m-ésimo plano de la imagen de entrada
- `pamslice -r n -x ruta/a/imagen.pam` : produce la salida en un formato adecuado para la entrada a un `xmgr` para la visualización

## Links
- More information: https://netpbm.sourceforge.net/doc/pamslice.html

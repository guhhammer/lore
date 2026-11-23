# pr

Paginación o disposición en columnas de archivos para su impresión.

## Usage
- `pr ruta/al/archivo1 ruta/al/archivo2 ...` : imprime varios archivos con un encabezado y un pie de página predeterminados
- `pr -h "encabezado" ruta/al/archivo1 ruta/al/archivo2 ...` : imprime con un encabezado centrado personalizado
- `pr -n -D "formato" ruta/al/archivo1 ruta/al/archivo2 ...` : imprime con líneas numeradas y un formato de fecha personalizado
- `pr -m -T ruta/al/archivo1 ruta/al/archivo2 ...` : imprime todos los archivos juntos, uno en cada columna, sin encabezado ni pie de página
- `pr +2:5 -l longitud_página ruta/al/archivo1 ruta/al/archivo2 ...` : imprime, comenzando en la página 2 hasta la página 5, con una longitud de página determinada (incluidos el encabezado y el pie de página)
- `pr -o sangrado -W ancho ruta/al/archivo1 ruta/al/archivo2 ...` : imprime con un sangrado para cada línea y un ancho de página personalizado truncado

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/pr-invocation.html

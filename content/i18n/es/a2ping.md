# a2ping

Convierte imágenes en archivos EPS o PDF.

## Usage
- `a2ping ruta/a/imagen.ext ruta/a/salida.pdf` : convierte una imagen a PDF (Nota: Es opcional especificar un nombre de archivo de salida)
- `a2ping --nocompress none|zip|best|flate ruta/al/archivo` : comprime el documento utilizando el método especificado
- `a2ping --nohires ruta/al/archivo` : escanea HiResBoundingBox si está presente (Nota: por defecto es sí)
- `a2ping --below ruta/al/archivo` : permite el contenido de la página por debajo y a la izquierda del origen (Nota: por defecto es no)
- `a2ping --gsextra argumentos ruta/al/archivo` : pasa argumentos adicionales a `gs`
- `a2ping --extra argumentos ruta/al/archivo` : pasa argumentos adicionales a un programa externo (por ejemplo, `pdftops`)
- `a2ping -h` : muestra la ayuda

## Links
- More information: https://manned.org/a2ping

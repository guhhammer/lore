# htmlq

Utiliza selectores CSS para extraer contenido de archivos HTML.

## Usage
- `cat ruta/al/archivo.html | htmlq '.card'` : devuelve todos los elementos de la clase `card`
- `cat ruta/al/archivo.html | htmlq -t 'p:primer-del-tipo'` : obtiene el contenido del texto del primer párrafo
- `cat ruta/al/archivo.html | htmlq -a href 'a'` : encuentra todos los enlaces de una página
- `cat ruta/al/archivo.html | htmlq -r 'img' -r 'svg'` : elimina todas las imágenes y archivos SVG de una página
- `htmlq -p -f ruta/al/archivo.html -o ruta/a/salida.html` : impresión bonita y escritura de la salida en un archivo

## Links
- More information: https://github.com/mgdm/htmlq#usage

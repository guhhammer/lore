# bat

Imprime y concatena archivos. Un clon de `cat` con resaltado de sintaxis e integración con Git.

## Usage
- `bat ruta/al/archivo1 ruta/al/archivo2 ...` : imprime bellamente (pretty print) el contenido de uno o más archivos en `stdout`
- `bat ruta/al/archivo1 ruta/al/archivo2 ... >ruta/al/archivo_destino` : concatena varios archivos en el archivo destino
- `bat -pp ruta/al/archivo` : elimina decoraciones y desactiva la paginación
- `bat -H 10|5:10|:10|10:|10:+5 ruta/al/archivo` : resalta una línea específica o un rango de líneas con un color de fondo diferente
- `bat -A ruta/al/archivo` : muestra caracteres no imprimibles como espacio, tabulador o nueva línea
- `bat -n ruta/al/archivo` : elimina todos los adornos excepto los números de línea en la salida
- `bat -l json ruta/al/archivo.json` : resalta sintácticamente un archivo JSON estableciendo explícitamente el lenguaje
- `bat -L` : muestra todos los lenguajes soportados

## Links
- More information: https://manned.org/bat

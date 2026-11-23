# sort

Ordena líneas de archivos de texto.

## Usage
- `sort ruta/al/archivo` : ordena un archivo en orden ascendente
- `sort -r ruta/al/archivo` : ordena un archivo en orden descendente
- `sort -f ruta/al/archivo` : ordena un archivo sin distinguir entre mayúsculas y minúsculas
- `sort -n ruta/al/archivo` : ordena un archivo usando orden numérico en lugar de alfabético
- `sort -t : -k 3n /etc/passwd` : ordena `/etc/passwd` por el tercer campo de cada línea numéricamente, usando `:` como separador de campos
- `sort -t : -k 3,3n -k 4,4g /etc/passwd` : como el anterior, pero cuando los elementos del tercer campo son iguales, ordena por el cuarto campo con números con exponentes
- `sort -u ruta/al/archivo` : ordena un archivo conservando únicamente las líneas únicas
- `sort -o ruta/al/archivo_de_salida ruta/al/archivo_de_entrada` : ordena un archivo, imprimiendo la salida en un archivo específico (puede usarse para ordenar un archivo en el sitio)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sort-invocation.html

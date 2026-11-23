# gocr

Herramienta de reconocimiento óptico de caracteres. Reconoce caracteres utilizando su motor y solicita al usuario patrones desconocidos para almacenarlos en una base de datos.

## Usage
- `gocr -m 130 -p ruta/al/directorio_db -i ruta/a/imagen_entrada.png -o ruta/al/archivo_salida.txt` : reconoce caracteres en una imagen y los escribe en un archivo. Coloca la base de datos en una carpeta existente para que no se omita su uso. modo 130 significa crear, usar y extender la base de datos
- `gocr -m 130 -p ruta/al/directorio_db -i ruta/a/imagen_entrada.png -o ruta/al/archivo_salida.txt -C "0..9"` : reconoce caracteres y asume que todos son números
- `gocr -m 130 -p ruta/al/directorio_db -i ruta/a/imagen_entrada.png -o ruta/al/archivo_salida.txt -a 100` : reconoce caracteres con certeza del 100% (los caracteres tienen una mayor probabilidad de ser tratados como desconocidos)

## Links
- More information: https://manned.org/gocr

# yaa

Crea y manipula archivos YAA.

## Usage
- `yaa archive -d ruta/al/directorio -o ruta/al/archivo_de_salida.yaa` : crea un archivo a partir de un directorio
- `yaa archive -i ruta/al/archivo -o ruta/al/archivo_de_salida.yaa` : crea un archivo a partir de un fichero
- `yaa extract -i ruta/al/archivo.yaa` : extrae un archivo al directorio actual
- `yaa list -i ruta/al/archivo.yaa` : lista el contenido de un archivo
- `yaa archive -a algorithm -d ruta/al/directorio -o ruta/al/archivo_de_salida.yaa` : crea un archivo con un algoritmo de compresión específico
- `yaa archive -b 8m -d ruta/al/directorio -o ruta/al/archivo_de_salida.yaa` : crea un archivo con un tamaño de bloque de 8 MB

## Links
- More information: https://www.manpagez.com/man/1/yaa/

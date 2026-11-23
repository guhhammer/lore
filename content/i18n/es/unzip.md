# unzip

Extrae archivos/directorios de archivos Zip.

## Usage
- `unzip ruta/al/archivo1.zip ruta/al/archivo2.zip ...` : extrae todos los archivos/directorios de archivos específicos en el directorio actual
- `unzip ruta/al/archivo1.zip ruta/al/archivo2.zip ... -d ruta/al/resultado` : extrae los archivos/directorios de archivos a una ruta específica
- `unzip -c ruta/al/archivo1.zip ruta/al/archivo2.zip ...` : extrae los archivos/directorios de archivos a `stdout` junto con los nombres de archivos extraídos
- `unzip -O gbk ruta/al/archivo1.zip ruta/al/archivo2.zip ...` : extrae un archivo creado en Windows, que contiene archivos con caracteres no ASCII en su nombre (por ejemplo, caracteres chinos o japoneses)
- `unzip -l ruta/al/archivo.zip` : enumera los contenidos de un archivo específico sin extraerlos
- `unzip -j ruta/al/archivo.zip ruta/al/archivo1_en_archivo ruta/al/archivo2_en_archivo ...` : extrae un archivo específico de un archivo

## Links
- More information: https://manned.org/unzip

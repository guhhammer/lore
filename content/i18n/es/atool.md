# atool

Un script para gestionar archivos comprimidos de varios tipos. `atool` utiliza programas de compresión externos, pero proporciona una interfaz de línea de comandos coherente para listar, extraer, crear y gestionar archivos comprimidos.

## Usage
- `atool -l ruta/al/archivo.zip` : lista archivos en un archivo comprimido
- `atool -x archivo.tar.gz` : extrae un archivo (crea un subdirectorio de forma segura si es necesario)
- `atool -X ruta/al/directorio_de_salida archivo.rar` : extrae un archivo a un directorio específico
- `atool -c archivo.tar ruta/al/archivo_en_archivo.txt` : muestra el contenido de un archivo específico de un archivo en la salida estándar (como `cat`)
- `atool -a archivo_nuevo.zip ruta/al/archivo1 ruta/al/archivo2 ...` : crea un nuevo archivo a partir de archivos y/o directorios especificados
- `atool -l -p archivo_grande.tar.bz2` : lista los archivos de un archivo y envía la salida a través de un paginador
- `atool -x -e archivo1.zip archivo2.tar.gz *.rar` : extrae varios archivos a la vez (cada uno a su propio subdirectorio si es necesario)
- `atool -r archivo_antiguo.tar.gz archivo_nuevo.tar.7z` : reempaqueta un archivo de un formato a otro (por ejemplo, de `.tar.gz` a `.tar.7z`)

## Links
- More information: https://manned.org/atool

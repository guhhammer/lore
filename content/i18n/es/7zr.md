# 7zr

Archivador de ficheros con un alto ratio de compresión. Similar a `7z` excepto que sólo soporta ficheros 7z.

## Usage
- `7zr a ruta/al/archivo.7z ruta/al/archivo_o_directorio` : archiva un archivo o directorio
- `7zr a ruta/al/archivo.7z -pcontraseña -mhe=on ruta/al/archivo.7z` : cifra un archivo existente (incluidos los nombres de los archivos)
- `7zr x ruta/al/archivo.7z` : extrae un archivo conservando la estructura de directorios original
- `7zr x ruta/al/archivo.7z -oruta/de/salida` : extrae un archivo a un directorio específico
- `7zr x ruta/al/archivo.7z -so` : extrae un archivo a `stdout`
- `7zr l ruta/al/archivo.7z` : lista el contenido de un archivo
- `7zr a ruta/al/archivo.7z -mx=0|1|3|5|7|9 ruta/al/archivo_o_directorio` : establece el nivel de compresión (más alto significa más compresión, pero más lento)

## Links
- More information: https://manned.org/7zr

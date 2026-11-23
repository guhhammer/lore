# 7za

Archivador de archivos con una alta relación de compresión. Similar a `7z`, salvo que admite menos tipos de archivos pero es multiplataforma.

## Usage
- `7za a ruta/al/archivo.7z ruta/al/archivo_o_directorio` : archiva un archivo o directorio
- `7za a ruta/al/encriptado.7z -pcontraseña -mhe=on ruta/al/archivo.7z` : encripta un archivo existente (incluyendo nombres de archivos)
- `7za x ruta/al/archivo.7z` : extrae un archivo preservando la estructura de directorios originales
- `7za x ruta/al/archivo.7z -oruta/de/salida` : extrae un archivo a un directorio específico
- `7za x ruta/al/archivo.7z -so` : extrae un archivo a `stdout`
- `7za a -t7z|bzip2|gzip|lzip|tar|... ruta/al/archivo.7z ruta/al/archivo_o_directorio` : archiva usando un tipo de archivo específico
- `7za l ruta/al/archivo.7z` : lista los contenidos de un archivo
- `7za a ruta/al/archivo.7z -mx=0|1|3|5|7|9 ruta/al/archivo_o_directorio` : establece el nivel de compresión (más alto significa más compresión, pero más lenta)

## Links
- More information: https://manned.org/7za

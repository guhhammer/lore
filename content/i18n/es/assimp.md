# assimp

Cliente de línea de comandos para la biblioteca Open Asset Import. Admite la carga de más de 40 formatos de archivo 3D y la exportación a varios formatos 3D populares.

## Usage
- `assimp listext` : lista todos los formatos de importación soportados
- `assimp listexport` : lista todos los formatos de exportación compatibles
- `assimp export archivo_entrada.stl archivo_salida.obj` : convierte un archivo a uno de los formatos de salida soportados, utilizando los parámetros por defecto
- `assimp export archivo_entrada.stl archivo_salida.obj parametros` : convierte un archivo utilizando parámetros personalizados (el archivo `dox_cmd.h` en el código fuente de assimp enumera los parámetros disponibles)
- `assimp info ruta/al/archivo` : muestra un resumen del contenido de un archivo 3D
- `assimp help` : muestra la ayuda
- `assimp subcomando --help` : muestra información de ayuda sobre un subcomando concreto (por ejemplo, los parámetros específicos del mismo)

## Links
- More information: https://manned.org/assimp

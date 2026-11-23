# delta

Un visor para Git y salida diff.

## Usage
- `delta ruta/al/archivo_antiguo_o_directorio ruta/al/archivo_o_directorio_nuevo` : compara archivos o directorios
- `delta -n ruta/al/archivo_o_directorio_antiguo ruta/al/archivo_o_directorio_nuevo` : compara archivos o directorios, mostrando los números de línea
- `delta -s ruta/al/archivo_o_directorio_antiguo ruta/al/archivo_o_directorio_nuevo` : compara archivos o directorios, mostrando las diferencias una al lado de la otra
- `delta --no-gitconfig ruta/al/archivo_o_directorio_antiguo ruta/al/archivo_o_directorio_nuevo` : compara archivos o directorios, ignorando cualquier configuración de Git
- `delta --hyperlinks ruta/al/archivo_o_directorio_antiguo ruta/al/archivo_o_directorio_nuevo` : compara, mostrando los hash de las confirmaciones, los nombres de los archivos y los números de línea como hipervínculos, de acuerdo con la especificación de hipervínculos para emuladores de terminal
- `delta --show-config` : muestra la configuración actual
- `delta --list-languages` : muestra los idiomas compatibles y las extensiones de archivo asociadas

## Links
- More information: https://dandavison.github.io/delta/full---help-output.html

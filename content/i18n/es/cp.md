# cp

Copia archivos y directorios.

## Usage
- `cp ruta/al/archivo_de_origen.ext ruta/al/archivo_destino.ext` : copia un archivo a otra ubicación
- `cp ruta/al/archivo_de_origen.ext ruta/al/directorio_destino` : copia un archivo en otro directorio, manteniendo el nombre del archivo
- `cp -r ruta/al/directorio_de_origen ruta/al/directorio_de_destino` : copia recursivamente el contenido de un directorio a otra ubicación (si el destino existe, el directorio se copia dentro de él)
- `cp -vr ruta/al/directorio_de_origen ruta/al/directorio_de_destino` : copia un directorio de forma recursiva, en modo detallado (muestra los archivos a medida que se copian)
- `cp -t ruta/al/directorio_de_destino ruta/al/archivo1 ruta/al/archivo2 ...` : copia varios archivos a la vez en un directorio
- `cp -i *.txt ruta/al/directorio_de_objetivo` : copia archivos de texto a otra ubicación, en modo interactivo (pregunta al usuario antes de sobrescribir)
- `cp -L enlace ruta/al/directorio_de_destino` : sigue los enlaces simbólicos antes de copiar
- `cp -t ruta/al/directorio_de_destino ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : utiliza el primer argumento como directorio de destino (útil para `xargs ... | cp -t destination_directory`)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html

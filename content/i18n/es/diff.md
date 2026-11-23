# diff

Compara archivos y directorios.

## Usage
- `diff archivo_viejo archivo_nuevo` : compara archivos (lista los cambios para convertir `archivo_viejo` en `archivo_nuevo`)
- `diff -w archivo_viejo archivo_nuevo` : compara archivos, ignorando los espacios en blanco
- `diff -y archivo_viejo archivo_nuevo` : compara archivos, mostrando las diferencias lado a lado
- `diff -u archivo_viejo archivo_nuevo` : compara archivos, mostrando las diferencias en formato unificado (como el que usa `git diff`)
- `diff -r directorio_viejo directorio_nuevo` : compara directorios de forma recursiva (muestra los nombres de los archivos/directorios que difieran y los cambios realizados en los archivos)
- `diff -r -q directorio_viejo directorio_nuevo` : compara directorios, mostrando solo los nombres de los archivos que difieren
- `diff -a -u -N archivo_viejo archivo_nuevo >diff.patch` : crea un archivo de revisión para Git a partir de las diferencias entre dos archivos de texto, tratando los archivos inexistentes como vacíos
- `diff -d --color=always archivo_viejo archivo_nuevo` : compara archivos, mostrando la salida en color y se esfuerza por encontrar el conjunto más pequeño de cambios

## Links
- More information: https://manned.org/diff

# git diff

Muestra los cambios en los archivos rastreados.

## Usage
- `git diff` : muestra cambios no preparados
- `git diff HEAD` : muestra todos los cambios no confirmados (incluyendo los preparados)
- `git diff --staged` : muestra sólo los cambios preparados (añadidos, pero aún no confirmados)
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : muestra los cambios de todos los confirmados desde una fecha/hora dada (una expresión de fecha, por ejemplo "1 week 2 days" o una fecha ISO)
- `git diff --stat confirmación` : muestra estadísticas de diff, como archivos cambiados, histograma y total de inserciones/eliminaciones de líneas
- `git diff --summary confirmación` : muestra un resumen de creaciones de archivos, renombramientos y cambios de modo desde una confirmación dada
- `git diff rama_1..rama_2 ruta/al/archivo` : compara un único archivo entre dos ramas o confirmaciones
- `git diff rama:ruta/al/archivo2 ruta/al/archivo1` : compara distintos archivos de la rama actual con otra rama

## Links
- More information: https://git-scm.com/docs/git-diff

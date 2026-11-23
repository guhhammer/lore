# git show

Muestra varios tipos de objetos Git (confirmaciones, etiquetas, etcétera).

## Usage
- `git show` : muestra información sobre la última confirmación (hash, mensaje, cambios y otros metadatos)
- `git show confirmación` : muestra información de una confirmación específica
- `git show etiqueta` : muestra información de la confirmación asociada a una determinada etiqueta
- `git show rama~3` : muestra información de la tercera confirmación desde la punta de una rama
- `git show --oneline -s confirmación` : muestra el mensaje de una confirmación en una única línea, eliminando el resultado de la diferencia
- `git show --stat confirmación` : muestra solo estadísticas (caracteres agregados o removidos) de los archivos modificados
- `git show --summary confirmación` : muestra solo la lista de archivos agregados, renombrados o eliminados
- `git show revisión:ruta/al/archivo` : muestra el contenido de un archivo en una revisión específica (por ej., una rama, una etiqueta o una confirmación)

## Links
- More information: https://git-scm.com/docs/git-show

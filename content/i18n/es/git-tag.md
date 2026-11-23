# git tag

Crea, muestra, borra o verifica etiquetas. Una etiqueta (tag) es una referencia estática a una confirmación (commit).

## Usage
- `git tag` : muestra todas las etiquetas
- `git tag nombre_de_la_etiqueta` : crea una etiqueta con el nombre especificado a partir de la confirmación actual
- `git tag nombre_de_la_etiqueta confirmación` : crea una etiqueta con el nombre especificado a partir de la confirmación señalada
- `git tag nombre_de_la_etiqueta -m mensaje_de_la_etiqueta` : crea una etiqueta anotada con el mensaje especificado
- `git tag -d nombre_de_la_etiqueta` : elimina la etiqueta con el nombre especificado
- `git fetch -t` : obtén las etiquetas actualizadas del remoto (remote)
- `git push origin tag nombre_de_la_etiqueta` : envía una etiqueta al remoto
- `git tag --contains confirmación` : muestra todas las etiquetas cuyos ancestros incluyen una confirmación específica

## Links
- More information: https://git-scm.com/docs/git-tag

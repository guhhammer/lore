# git log

Muestra un historial de confirmaciones.

## Usage
- `git log` : muestra la secuencia de confirmaciones comenzando desde el actual, en orden cronológico inverso, del respositorio de Git en el directorio de trabajo actual
- `git log -p ruta/al/archivo_o_directorio` : muestra el historial de un archivo o directorio específico, incluyendo las diferencias
- `git log --stat` : muestra un resumen de los archivos, o archivo, cambiados en cada confirmación
- `git log --oneline --graph` : muestra un gráfico de las confirmaciones en la rama actual, utilizando solo la primer línea del mensaje de cada uno
- `git log --oneline --decorate --all --graph` : muestra un gráfico de todos las confirmaciones, etiquetas y ramas en todo el repositorio
- `git log -i --grep cadena_a_buscar` : muestra solo las confirmaciones cuyo mensaje incluye una cadena dada (no diferencia entre mayúsculas y minúsculas)
- `git log -n número --author "autor"` : muestra las últimas N confirmaciones de determinado autor
- `git log --before "2017-01-29" --after "2017-01-17"` : muestra las confirmaciones entre dos fechas (yyyy-mm-dd)

## Links
- More information: https://git-scm.com/docs/git-log

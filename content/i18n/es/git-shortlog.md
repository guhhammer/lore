# git shortlog

Resume la salida de `git log`.

## Usage
- `git shortlog` : muestra un resumen de todas las confirmaciones realizadas, agrupadas alfabéticamente por autor
- `git shortlog -n` : muestra un resumen de todas las confirmaciones realizadas, agrupadas por el número de confirmaciones realizadas
- `git shortlog -c` : muestra un resumen de todas las confirmaciones realizadas, agrupadas por la identidad de quien realiza la confirmación (usuario y correo electrónico)
- `git shortlog HEAD~5..HEAD` : muestra un resumen de las últimas cinco confirmaciones (p. e., un rango de revisiones específico)
- `git shortlog -sne` : muestra todos los usuarios, correos electrónicos y número de confirmaciones en la rama actual
- `git shortlog -sne --all` : muestra todos los usuarios, correos electrónicos y número de confirmaciones en todas las ramas

## Links
- More information: https://git-scm.com/docs/git-shortlog

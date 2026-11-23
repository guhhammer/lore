# git check-ignore

Analiza y depura los archivos que Git debe ignorar / excluir (`.gitignore`).

## Usage
- `git check-ignore ruta/al/archivo_o_directorio` : comprueba si un archivo o directorio es ignorado
- `git check-ignore ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : comprueba si varios archivos o directorios son ignorados
- `git <ruta/al/archivo_lista check-ignore --stdin` : usa nombres de rutas, uno por línea, a partir de la entrada estandar (`stdin`)
- `git check-ignore --no-index ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : comprueba sin leer el índice (se utiliza para depurar por qué las rutas fueron rastreadas y no ignoradas)
- `git check-ignore -v ruta/al/archivo_o_directorio1 ruta/al/archivo_o_directorio2 ...` : incluye detalles sobre el patrón de coincidencia para cada ruta

## Links
- More information: https://git-scm.com/docs/git-check-ignore

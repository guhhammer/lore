# vimdiff

Abre dos o más archivos en Vim y muestra las diferencias entre ellos.

## Usage
- `vimdiff ruta/al/archivo1 ruta/al/archivo2` : abre dos archivos y muestra las diferencias
- `<Ctrl w><h>|<l>` : mueve el cursor a la ventana de la izquierda|derecha
- `<[><c>` : salta a la diferencia previa
- `<]><c>` : salta a la siguiente diferencia
- `<d><o>` : copia la diferencia resaltada de la otra ventana a la ventana actual
- `<d><p>` : copia la diferencia resaltada de la ventana actual a la otra ventana
- `<:>diffupdate` : actualiza todos los resaltados y folds (plegados de texto)
- `<z><a>` : alterna la apertura/cierre de la fold (plegado de texto) de código resaltada

## Links
- More information: https://www.vim.org/

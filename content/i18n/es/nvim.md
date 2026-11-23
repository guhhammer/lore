# nvim

Neovim, un editor de texto para programadores basado en Vim, ofrece varios modos para diferentes tipos de manipulación de texto. Al pulsar `<i>` en modo normal se entra en modo de inserción. `<Esc>` o `<Ctrl c>` vuelve al modo normal, lo cual no permite la inserción de texto normal.

## Usage
- `nvim ruta/al/archivo` : abre un archivo
- `<Esc><i>` : entra en el modo de edición de texto (modo de inserción)
- `<Esc><y><y>|<d><d>` : copia ("yank") o corta ("delete") la línea actual (pégala con `<p>`)
- `<Esc><u>` : entra en modo normal y deshace la última operación
- `<Esc></>patrón_búsqueda<Enter>` : busca un patrón en el archivo (pulsa `<n>`/`<N>` para ir a la coincidencia siguiente/anterior)
- `<Esc><:>%s/expresión_regular/texto_reemplazo/g<Enter>` : realiza una sustitución `regex` en todo el archivo
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : entra en modo normal y guarda (escribe) el archivo, y sale
- `<Esc><:>q!<Enter>` : sale sin guardar

## Links
- More information: https://neovim.io/

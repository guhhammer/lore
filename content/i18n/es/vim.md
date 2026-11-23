# vim

Vim (Vi IMproved), un editor de texto para la línea de comandos, que proporciona varios modos para diferentes tipos de manipulación de texto. Pulsando `<i>` entra en el modo insertar. `<Esc>` regresa al modo normal, permitiendo el uso de comandos Vim.

## Usage
- `vim ruta/al/archivo` : abre un archivo
- `vim +número_de_línea ruta/al/archivo` : abre un archivo en un número de línea especificado
- `<:>help<Enter>` : muestra el manual de Vim
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : guarda y sale
- `<Esc><u>` : deshace la última operación
- `</>patrón_a_buscar<Enter>` : busca un patrón en el archivo (pulsa `<n>`/`<N>` para ir a la próxima/previa coincidencia)
- `<:>%s/expresión_regular/reemplazo/g<Enter>` : realiza una sustitución de una `regex` en el archivo completo
- `<:>set nu<Enter>` : muestra los números de línea

## Links
- More information: https://www.vim.org/

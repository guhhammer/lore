# history

Historial de la línea de comandos.

## Usage
- `history` : muestra el historial de comandos junto a su número de línea
- `history 20` : muestra los últimos 20 comandos (en Zsh muestra todos los comandos a partir del 20)
- `history -d|f|i|E` : muestra el historial con marcas de tiempo en diferentes formatos (solo disponible en Zsh)
- `history -c` : limpia el historial de comandos (solo para la interfaz de comandos actual)
- `history -w` : sobrescribe el archivo histórico con el historial de la sesión actual (comúnmente se combina con `history -c` para limpiar el historial)
- `history -d índice` : borra la entrada del historial en el índice especificado
- `history -s comando` : añade un comando al historial sin ejecutarlo

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history

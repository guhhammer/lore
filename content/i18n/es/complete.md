# complete

Obtiene y establece las reglas de autocompletado de argumentos de los comandos del shell en Bash. Las terminaciones especificadas se invocarán cuando se pulse `<Tab>` en Bash.

## Usage
- `complete -F función comando` : establece los argumentos de un comando para autocompletar a través de una función (la respuesta de completado se envía en la variable `$COMPREPLY`)
- `complete -C comando_de_autocompletado comando` : establece los argumentos de un comando para autocompletar a través de otro comando (`$1` es el comando, `$2` es el argumento sobre el que está el cursor y `$3` es el argumento que precede al cursor)
- `complete -b comando` : configura los argumentos de un comando para que se autocompleten con los componentes del shell
- `complete -o nospace -F función comando` : aplica el autocompletado sin añadir un espacio a la palabra completada
- `complete -p` : lista todas las especificaciones completas cargadas
- `complete -p comando` : lista de especificaciones completas cargadas para un comando

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-complete

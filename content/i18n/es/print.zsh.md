# print.zsh

Comando integrado de Z Shell (`zsh`). Muestra los argumentos, de forma similar a `echo`.

## Usage
- `print "Hello" "World"` : muestra la entrada
- `print -l "Línea 1" "Línea 2" "Línea 3"` : imprime separado por saltos de línea
- `print -n "Hola"; print "Mundo"` : imprime sin salto de línea al final
- `print -e "Línea 1\nLínea 2"` : habilita caracteres de escape con barra invertida
- `print -f "%s tiene %d años.\n" "Alice" 30` : imprime argumentos tal y como se describe en `printf` (para una mayor portabilidad entre intérprete de comandos, considera utilizar el comando `printf` en su lugar)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html

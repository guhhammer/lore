# printf

Formatea e imprime texto.

## Usage
- `printf "%s\n" "Hola mundo"` : imprime un mensaje de texto
- `printf "\e[1;34m%.3d\e[0m\n" 42` : imprime un número entero en negrita azul
- `printf "\u20AC %.2f\n" 123.4` : imprime un número flotante con el signo del euro Unicode
- `printf "var1: %s\tvar2: %s\n" "$VAR1" "$VAR2"` : imprime un mensaje de texto compuesto con variables de entorno
- `printf -v myvar "Esto es %s = %d\n" "un año" 2016` : almacena un mensaje formateado en una variable (no funciona en Zsh)
- `printf "hex=%x octal=%o scientific=%e\n" 0xFF 0377 100000` : imprime un número hexadecimal, octal y científico

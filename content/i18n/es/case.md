# case

Construcción de Bash para crear sentencias condicionales multi-elección.

## Usage
- `case $COUNTRULE in palabras) wc --words LÉAME ;; líneas) wc --lines LÉAME ;; esac` : compara una variable con literales de cadena para decidir qué comando ejecutar
- `case $COUNTRULE in wW]) wc --words LÉAME ;; lL]) wc --lines LÉAME ;; *) echo "¿qué?" ;; esac` : combina patrones con |, usar * como patrón de reserva
- `case $ANIMAL in gato) echo "Es un gato" ;;& gato|perro) echo "Es un gato o un perro" ;;& *) echo "Fallback" ;; esac` : permite la coincidencia de múltiples patrones
- `case $ANIMAL in gato) echo "Es un gato" ;& perro) echo "O es un perro o es un gato" ;& *) echo "Fallback" ;; esac` : continúa con los comandos del siguiente patrón sin comprobar el patrón
- `help case` : muestra la ayuda

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-case

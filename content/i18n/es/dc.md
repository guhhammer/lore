# dc

Una calculadora de precisión arbitraria. Utiliza notación polaca inversa (RPN).

## Usage
- `dc` : inicia una sesión interactiva
- `dc ruta/a/script.dc` : ejecuta un script
- `dc -e '10 k 5 3 / p'` : calcula una expresión con la escala especificada
- `dc -e '4 5 * 17 - p'` : calcula 4 veces 5 (4 5 *), resta 17 (17 -), e imprime la salida
- `dc -e '7 k 5 _3 / p'` : especifica el número de decimales a 7 (7 k), calcula 5 dividido por -3 (5 _3 /) y lo imprime
- `dc -e '100 k 5 v 1 + 2 / p'` : calcula la proporción áurea, phi: establece el número de decimales a 100 (100 k), raíz cuadrada de 5 (5 v) más 1 (1 +), dividido por 2 (2 /), e imprime el resultado

## Links
- More information: https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html

# expr

Evalúa expresiones y manipula cadenas.

## Usage
- `expr length "cadena"` : obtiene la longitud de una cadena específica
- `expr substr "cadena" desde longitud` : obtiene la subcadena de una cadena con una longitud específica
- `expr match "cadena" 'patrón'` : empareja una subcadena específica frente a un patrón
- `expr index "cadena" "caracteres"` : obtiene la primera posición de un caracter de un conjunto específico en una cadena
- `expr expresión1 +|-|*|/|% expresión2` : calcula una expresión matemática específica
- `expr expresión1 \| expresión2` : obtiene la primera expresión si su valor no es cero y no nulo, de otro modo, obtiene el segundo
- `expr expresión1 \& expresión2` : obtiene la primera expresión si ambas expresiones no son cero y no nulas de otro modo obtiene cero

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/expr-invocation.html

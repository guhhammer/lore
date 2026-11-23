# comm

Selecciona o rechaza las líneas comunes a dos ficheros. Ambos ficheros deben estar ordenados.

## Usage
- `comm fichero1 fichero2` : produce tres columnas separadas por tabuladores: líneas solo en el primer archivo, líneas solo en el segundo archivo y líneas comunes
- `comm -12 fichero1 fichero2` : imprime solo las líneas comunes a ambos archivos
- `cat fichero1 | comm -12 - fichero2` : imprime solo las líneas comunes a ambos archivos, leyendo un archivo desde `stdin`
- `comm -23 fichero1 fichero2 >solo_fichero1` : obtiene las líneas que solo se encuentran en el primer fichero, guardando el resultado en un tercer fichero
- `comm -13 <(sort fichero1) <(sort fichero2)` : imprime las líneas solo encontradas en el segundo fichero, cuando los ficheros no están ordenados

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/comm-invocation.html

# nettop

Muestra información actualizada sobre la red.

## Usage
- `nettop` : supervisa sockets TCP y UDP de todas las interfaces
- `nettop -m tcp -t loopback` : supervisa sockets TCP de las interfaces de bucle invertido
- `nettop -p "process_id|process_name"` : supervisa un proceso específico
- `nettop -P` : muestra un resumen por proceso
- `nettop -l 10` : muestra 10 muestras de información de red
- `nettop -d -s 5` : supervisa los cambios cada 5 segundos
- `<h>` : mientras se ejecuta nettop, muestra los comandos interactivos
- `nettop -h` : muestra la ayuda

## Links
- More information: https://keith.github.io/xcode-man-pages/nettop.1.html

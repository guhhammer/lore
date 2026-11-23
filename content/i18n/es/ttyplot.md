# ttyplot

Una utilidad de trazado en tiempo real para la línea de comandos con entrada de datos desde `stdin`.

## Usage
- `{ echo 1 2 3; cat; } | ttyplot` : muestra los valores `1`, `2` y `3` (`cat` evita que ttyplot salga)
- `{ echo 1 2 3; cat; } | ttyplot -t título -u unidad` : establece un título específico y unidad
- `{ while true; do echo $RANDOM; sleep 1; done } | ttyplot` : utiliza un bucle de tiempo para trazar continuamente valores aleatorios
- `ping 8.8.8.8 | sed -u 's/^.*time=//g; s/ ms//g' | ttyplot -t "ping a 8.8.8.8" -u ms` : analiza la salida de `ping` y la visualiza

## Links
- More information: https://github.com/tenox7/ttyplot

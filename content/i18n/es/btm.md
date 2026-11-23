# btm

Muestra información del sistema sobre la CPU, la memoria, los discos, la red y los procesos. Una alternativa mejorada a `top`.

## Usage
- `btm` : muestra el diseño predeterminado (CPU, memoria, temperaturas, disco, red y procesos)
- `btm -b` : habilita el modo básico, eliminando gráficos y condensando datos (similar a `top`)
- `btm -m` : utiliza puntos grandes en lugar de pequeños en los gráficos
- `btm --battery` : muestra también la carga de la batería y el estado de salud
- `btm -r 250 -t 30000` : actualiza cada 250 milisegundos y muestra los últimos 30 segundos en los gráficos

## Links
- More information: https://clementtsang.github.io/bottom/nightly/#usage-and-configuration

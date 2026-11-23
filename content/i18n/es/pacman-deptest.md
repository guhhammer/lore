# pacman-deptest

Comprueba cada dependencia especificada y devuelve una lista de las dependencias que no están satisfechas actualmente en el sistema.

## Usage
- `pacman -T paquete1 paquete2 ...` : muestra los nombres de los paquetes de las dependencias que no están instaladas
- `pacman -T "bash>=5"` : comprueba si el paquete instalado cumple con la versión mínima especificada
- `pacman -T "bash>5"` : comprueba si hay instalada una versión posterior de un paquete
- `pacman -Th` : muestra la ayuda

## Links
- More information: https://manned.org/pacman.8

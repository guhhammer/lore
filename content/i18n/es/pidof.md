# pidof

Obtiene el ID de un proceso a partir de su nombre.

## Usage
- `pidof bash` : muestra todos los ID de proceso con el nombre indicado
- `pidof -s bash` : muestra un único ID de proceso con el nombre indicado
- `pidof -x script.py` : muestra los ID de los procesos, incluidos los scripts, con el nombre indicado
- `kill $(pidof nombre)` : termina todos los procesos con el nombre indicado

## Links
- More information: https://manned.org/pidof

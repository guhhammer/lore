# turbostat

Informa de la topología del procesador, frecuencia, temperatura, potencia y estadísticas de inactividad.

## Usage
- `sudo turbostat` : muestra las estadísticas cada cinco segundos
- `sudo turbostat -i n_segundos` : muestra las estadísticas cada cierto número de segundos
- `sudo turbostat --quiet` : muestra información sin decodificar ni imprimir la cabecera de configuración del sistema
- `sudo turbostat --quiet -i 1 --cpu 0-cuenta_hilos_cpu --show "PkgWatt","Busy%","Core","CoreTmp","Thermal"` : muestra información útil sobre el CPU cada segundo, sin información de cabecera
- `turbostat --help` : muestra la ayuda

## Links
- More information: https://manned.org/turbostat

# tunelp

Establece varios parámetros para dispositivos de puerto paralelo para la solución de problemas o para un mejor rendimiento. Parte de `util-linux`.

## Usage
- `tunelp -s /dev/lp0` : comprueba el estado de un dispositivo de puerto paralelo
- `tunelp -r /dev/lp0` : restablece un puerto paralelo
- `tunelp -i 5 /dev/lp0` : utiliza un iRQ dado para un dispositivo, cada uno representando una línea de interrupción
- `tunelp -c veces -t tiempo_en_centisegundos /dev/lp0` : intenta imprimir varias veces un carácter en la impresora antes de permanecer inactiva durante un tiempo determinado
- `tunelp -a on|off` : activa o desactiva el abortar por error (desactivado por defecto)

## Links
- More information: https://manned.org/tunelp

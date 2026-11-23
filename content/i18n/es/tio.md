# tio

Supervisa e interactúa con puertos serie.

## Usage
- `tio /dev/ttyUSB0` : abre un puerto serie con la configuración predeterminada
- `tio -b 9600 /dev/ttyUSB0` : abre un puerto serie con una velocidad de transmisión específica
- `tio -L --log-file archivo_de_registro /dev/ttyUSB0` : abre un puerto serie y registra la salida en un archivo
- `tio --output-mode hex /dev/ttyUSB0` : abre un puerto serie y habilita la salida hexadecimal
- `tio -l` : muestra los puertos serie disponibles
- `<Ctrl t><q>` : sale de la sesión `tio`

## Links
- More information: https://github.com/tio/tio#3-usage

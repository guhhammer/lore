# minicom

Se comunica con la interfaz serie de un dispositivo.

## Usage
- `sudo minicom -D /dev/ttyXYZ` : abre un puerto serie determinado
- `sudo minicom -D /dev/ttyXYZ -b 115200` : abre un puerto serie determinado con una velocidad de transmisión determinada
- `sudo minicom -D /dev/ttyXYZ -s` : accede al menú de configuración antes de comunicarse con un puerto serie determinado
- `sudo minicom -D /dev/ttyXYZ -C ruta/al/archivo` : captura la salida de un puerto serie en un archivo
- `<Ctrl a><x><Enter>` : sale de minicom
- `minicom -h` : muestra la ayuda

## Links
- More information: https://manned.org/minicom

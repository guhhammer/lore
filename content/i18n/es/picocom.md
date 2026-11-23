# picocom

Programa mínimo para emular consolas serie.

## Usage
- `sudo picocom /dev/ttyXYZ` : se conecta a una consola serie con una velocidad de transmisión predeterminada de 9600
- `sudo picocom /dev/ttyXYZ -b tasa_de_baudios` : se conecta a una consola serie con una velocidad de transmisión especificada
- `sudo picocom /dev/ttyXYZ --imap lfcrlf` : asigna caracteres especiales (por ejemplo, `LF` a `CRLF`)
- `<Ctrl a><Ctrl x>` : sale de picocom
- `picocom -h` : muestra la ayuda

## Links
- More information: https://manned.org/picocom

# nvram

Manipula variables del firmware.

## Usage
- `nvram -p` : imprime todas las variables almacenadas en la NVRAM
- `nvram -xp` : imprime todas las variables almacenadas en la NVRAM usando el formato xML
- `sudo nvram nombre="valor"` : modifica el valor de una variable del firmware
- `sudo nvram -d nombre` : elimina una variable de firmware
- `sudo nvram -c` : clarifica todas las variables de firmware
- `sudo nvram -xf ruta/al/archivo.xml` : establece una variable de firmware de un xML file específico

## Links
- More information: https://keith.github.io/xcode-man-pages/nvram.8.html

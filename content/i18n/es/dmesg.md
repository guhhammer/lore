# dmesg

Escribe los mensajes del núcleo a la salida estándar.

## Usage
- `sudo dmesg` : muestra los mensajes del núcleo
- `sudo dmesg -l err` : muestra los mensajes de error del núcleo
- `sudo dmesg -w` : muestra los mensajes del núcleo y sigue leyendo los nuevos, similar a `tail -f` (disponible en los núcleos 3.5.0 y posteriores)
- `sudo dmesg | grep -i memory` : muestra cuanta memoria física hay disponible en este sistema
- `sudo dmesg | less` : muestra los mensajes del núcleo, página a página
- `sudo dmesg -T` : muestra los mensajes del núcleo con una estampilla temporal (disponible en los núcleos 3.5.0 y posteriores)
- `sudo dmesg -H` : muestra los mensajes del núcleo de forma legible para humanos (disponible en los núcleos 3.5.0 y posteriores)
- `sudo dmesg -L` : colorea la salida (disponible en los núcleos 3.5.0 y posteriores)

## Links
- More information: https://manned.org/dmesg

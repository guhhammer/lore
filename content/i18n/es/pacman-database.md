# pacman-database

Opera en la base de datos de paquetes de Arch Linux. Modifica ciertos atributos de los paquetes instalados.

## Usage
- `sudo pacman -D --asdeps paquete` : marca un paquete como instalado implícitamente
- `sudo pacman -D --asexplicit paquete` : marca un paquete como instalado explícitamente
- `pacman -Dk` : verifica que todas las dependencias del paquete estén instaladas
- `pacman -Dkk` : verifica los repositorios para asegurarse de que todas las dependencias especificadas estén disponibles
- `pacman -Dkq` : muestra solo mensajes de error
- `pacman -Dh` : muestra la ayuda

## Links
- More information: https://manned.org/pacman.8

# dpkg

Administrador de paquetes de Debian. Algunos subcomandos como `deb` tienen su propia documentación de uso.

## Usage
- `sudo dpkg -i ruta/al/archivo.deb` : instala un paquete
- `sudo dpkg -r paquete` : remueve un paquete
- `dpkg -l patrón` : lista los paquetes instalados
- `dpkg -L paquete` : lista los contenidos de un paquete
- `dpkg -c ruta/al/archivo.deb` : lista los contenidos de un archivo de paquete local
- `dpkg -S ruta/al/archivo` : averigua qué paquete posee un archivo
- `sudo dpkg -P paquete` : purga un paquete instalado o ya eliminado, incluyendo su configuración

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta

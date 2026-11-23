# aur

Construye paquetes desde el AUR y gestiona repositorios locales. Nota: Es necesario establecer un repositorio local en `/etc/pacman.conf` e instalar `vifm` para que funcione completamente.

## Usage
- `aur search palabra_clave` : busca un paquete en la base de datos del AUR
- `aur sync paquete` : descarga un paquete y sus dependencias desde el AUR, los compila y añade a un repositorio local
- `aur repo -l` : lista paquetes disponibles en tu repositorio local
- `aur sync -u` : actualiza los paquetes del repositorio local
- `aur sync --noview -n paquete` : instala un paquete sin ver los cambios en Vim y sin confirmar la instalación de dependencias

## Links
- More information: https://github.com/aurutils/aurutils

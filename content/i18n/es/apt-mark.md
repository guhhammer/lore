# apt-mark

Herramienta para cambiar el estado de los paquetes instalados.

## Usage
- `sudo apt-mark auto paquete` : marca un paquete como instalado automáticamente
- `sudo apt-mark hold paquete` : mantiene un paquete en su versión actual y evita que se actualice
- `sudo apt-mark unhold paquete` : permite que un paquete pueda ser actualizado de nuevo
- `apt-mark showmanual` : muestra los paquetes instalados manualmente
- `apt-mark showhold` : muestra los paquetes mantenidos que no son actualizados

## Links
- More information: https://manned.org/apt-mark

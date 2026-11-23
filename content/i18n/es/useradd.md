# useradd

Crea un usuario.

## Usage
- `sudo useradd usuario` : crea un usuario
- `sudo useradd -u id usuario` : crea un usuario con el ID de usuario específico
- `sudo useradd -s ruta/a/la/shell usuario` : crea un usuario con una línea de comando (shell) específica
- `sudo useradd -G grupo1,grupo2,... usuario` : crea un usuario perteneciente a grupos adicionales (ten en cuenta que no se colocan espacios en blanco)
- `sudo useradd -m usuario` : crea un usuario con el directorio home predeterminado
- `sudo useradd -k ruta/al/directorio_plantilla -m usuario` : crea un usuario con el directorio home con una copia de los archivos provenientes de un directorio plantilla
- `sudo useradd -r usuario` : crea un usuario del sistema sin directorio home

## Links
- More information: https://manned.org/useradd

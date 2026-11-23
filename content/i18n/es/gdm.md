# gdm

El gestor de pantalla GNOME (GDM) sustituye al X Display Manager (XDM).

## Usage
- `gdm` : ejecuta la aplicación GNOME Display Manager
- `gdm --nodaemon` : evita que `gdm` se ejecute como un proceso de fondo daemon
- `gdm --no-console` : desactiva la gestión de `gdm` de los servidores X de consola local para entornos sin pantalla o remotos
- `gdm --preserve-ld-vars` : evita la desinfección de variables de entorno que comienzan con `$LD_`
- `gdm --help` : muestra la ayuda
- `gdm --version` : muestra la versión

## Links
- More information: https://manned.org/gdm

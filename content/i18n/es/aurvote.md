# aurvote

Vota por paquetes en el repositorio de usuarios de Arch (AUR). Para poder votar, el archivo `~/.config/aurvote` debe existir y contener tus credenciales del AUR.

## Usage
- `aurvote --configure` : crea interactivamente el archivo `~/.config/aurvote` que contiene su nombre de usuario y contraseña del AUR
- `aurvote paquete1 paquete2 ...` : vota por uno o más paquetes del AUR
- `aurvote -u paquete1 paquete2 ...` : retira el voto de uno o más paquetes del AUR
- `aurvote -c paquete1 paquete2 ...` : verifica si uno o más paquetes del AUR ya han sido votados
- `aurvote -h` : muestra la ayuda

## Links
- More information: https://github.com/archlinuxfr/aurvote#name

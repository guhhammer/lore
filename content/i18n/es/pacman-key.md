# pacman-key

Script de encapsulado para GnuPG utilizado para gestionar el conjunto de claves de pacman.

## Usage
- `sudo pacman-key --init` : inicializa el conjunto de claves de `pacman`
- `sudo pacman-key --populate` : añade las claves predeterminadas de Arch Linux
- `pacman-key -l` : muestra las claves del portallaves público
- `sudo pacman-key -a ruta/al/archivo-de-claves.gpg` : añade las claves especificadas
- `sudo pacman-key -r "uid|name|email"` : recibe una clave de un servidor de claves
- `pacman-key -f "uid|name|email"` : muestra la huella digital de una clave específica
- `sudo pacman-key --lsign-key "uid|name|email"` : firma una clave importada localmente
- `sudo pacman-key -d "uid|name|email"` : elimina una clave específica

## Links
- More information: https://manned.org/pacman-key

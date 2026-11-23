# pacman-mirrors

Genera una lista de réplicas de `pacman` para Manjaro Linux. Cada vez que se ejecute `pacman-mirrors`, es necesario sincronizar la base de datos y actualizar el sistema con `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : genera una lista de réplicas utilizando la configuración predeterminada
- `pacman-mirrors --status` : obtiene el estado de las réplicas actuales
- `pacman-mirrors --get-branch` : muestra la rama actual
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : cambia a una rama diferente
- `sudo pacman-mirrors --geoip` : genera una lista de servidores de réplicas utilizando únicamente los de tu país

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors

# arch-chroot

Comando `chroot` mejorado para facilitar el proceso de instalación de Arch Linux.

## Usage
- `arch-chroot ruta/a/nueva/raíz` : inicia un intérprete de comandos interactivo (Bash por defecto) en un nuevo directorio raíz
- `arch-chroot -u usuario ruta/a/nueva/raíz` : especifica el usuario (distinto del actual) para ejecutar el intérprete de comandos
- `arch-chroot ruta/a/nueva/raíz comando argumentos_del_comando` : ejecuta un comando personalizado (en lugar de Bash) en el nuevo directorio raíz
- `arch-chroot ruta/a/nueva/raíz zsh` : especifica un intérprete de comandos distinto de Bash (en este caso, el paquete `zsh` debe estar instalado en el sistema de destino)

## Links
- More information: https://manned.org/arch-chroot

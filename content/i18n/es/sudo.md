# sudo

Ejecuta un comando como superusuario o como otro usuario.

## Usage
- `sudo less /var/log/syslog` : ejecuta un comando como superusuario
- `sudo -e /etc/fstab` : edita un archivo como superusuario con su editor predeterminado
- `sudo -u usuario -g grupo id -a` : ejecuta un comando como otro usuario y/o grupo
- `sudo !!` : repite el último comando precedido de `sudo` (solo en Bash, Zsh, etc.)
- `sudo -i` : inicia el intérprete de comandos predeterminado con privilegios de superusuario y ejecuta los archivos específicos de inicio de sesión (`.profile`, `.bash_profile`, etc.)
- `sudo -s` : inicia el intérprete de comandos predeterminado con privilegios de superusuario sin cambiar el entorno
- `sudo -i -u usuario` : inicia el intérprete de comandos predeterminado como el usuario especificado, cargando el entorno del usuario y leyendo los archivos específicos de inicio de sesión (`.profile`, `.bash_profile`, etc.)
- `sudo -ll` : muestra los comandos permitidos (y prohibidos) para el usuario que ejecuta el comando en formato extendido

## Links
- More information: https://www.sudo.ws/sudo.html

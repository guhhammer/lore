# last

Vea los últimos usuarios conectados.

## Usage
- `last` : muestra la información del último inicio de sesión (por ejemplo, nombre de usuario, terminal, tiempo de arranque, kernel) de todos los usuarios leída de `/var/log/wtmp`
- `last usuario` : lista la información de inicio de sesión de un usuario específico
- `last -n cuenta_inicios` : especifica cuántos de los últimos inicios de sesión mostrar
- `last -F -a` : muestra la fecha y hora completas de las entradas y, a continuación, muestra la columna del nombre de host en último lugar para evitar que se trunque
- `last usuario -i` : visualiza todos los inicios de sesión de un usuario específico y muestra la dirección IP en lugar del nombre de host
- `last -s -7days` : lista la información desde una fecha y hora determinadas
- `last reboot` : muestra todos los reinicios registrados (es decir, los últimos inicios de sesión del pseudousuario "reboot")
- `last -h` : muestra la ayuda

## Links
- More information: https://manned.org/last

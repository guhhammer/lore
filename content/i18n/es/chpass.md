# chpass

Añade o cambia la información de la base de datos del usuario, incluyendo el intérprete de comandos (shell) y la contraseña. Nota: no es posible cambiar la contraseña del usuario en sistemas Open Directory, utiliza `passwd` en su lugar.

## Usage
- `su -c chpass` : añade o cambia la información de la base de datos del usuario actual de forma interactiva
- `chpass -s ruta/al/shell` : establece un shell de inicio de sesión específico para el usuario actual
- `chpass -s ruta/al/shell usuario` : establece un inicio de sesión shell para un usuario específico
- `chpass -l ubicación -s ruta/al/shell usuario` : edita el registro de usuario en el nodo de directorio en la ubicación dada
- `chpass -u usuario_autenticado -s ruta/al/shell usuario` : utiliza el usuario_autenticado al identificarse en el nodo de directorio que contiene a cierto usuario

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass

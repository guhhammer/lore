# usermod

Modifica una cuenta de usuario.

## Usage
- `sudo usermod -l nuevo_nombre usuario` : cambia el nombre de un usuario
- `sudo usermod -u id usuario` : cambia el ID de un usuario
- `sudo usermod -s ruta/a/interfaz_comando usuario` : cambia la interfaz de comandos (shell) a un usuario
- `sudo usermod -a -G grupo1,grupo2 usuario` : añade un usuario a grupos suplementarios (ten en cuenta los espacios en blanco)
- `sudo usermod -m -d ruta/al/nuevo_home usuario` : cambia el directorio home de un usuario

## Links
- More information: https://manned.org/usermod

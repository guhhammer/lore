# lxc-create

Crea contenedores linux.

## Usage
- `sudo lxc-create -n contenedor -t download` : crea un contenedor interactivamente en `/var/lib/lxc/`
- `sudo lxc-create -P /ruta/al/directorio/ -n contenedor -t download` : crea un contenedor en un directorio de destino
- `sudo lxc-create -n nombre -t download -- -d nombre_distro -r versión_de_lanzamiento -a arch` : crea un contenedor pasando opciones a una plantilla
- `lxc-create -?` : muestra la ayuda

## Links
- More information: https://linuxcontainers.org/lxc/getting-started/

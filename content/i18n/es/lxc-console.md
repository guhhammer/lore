# lxc-console

Se conecta a un contenedor.

## Usage
- `agetty -L 38400 tty1` : inicia una consola en un contenedor
- `sudo lxc-console nombre_del_contenedor` : se conecta a una consola lxc
- `<Ctrl a><q>` : sale de `lxc-console`
- `lxc-console -?` : muestra la ayuda

## Links
- More information: https://linuxcontainers.org/lxc/manpages//man1/lxc-console.1.html

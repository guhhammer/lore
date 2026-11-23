# systemd-sysext

Activa o desactiva imágenes de extensión del sistema (system extension images).

## Usage
- `systemd-sysext list` : lista de imágenes de extensión instaladas
- `systemd-sysext merge` : combina imágenes de extensión del sistema en `/usr/` y `/opt/`
- `systemd-sysext` : comprueba el estado de fusión actual
- `systemd-sysext unmerge` : separa todas las imágenes de extensión del sistema actualmente instaladas en `/usr/` y `/opt/`
- `systemd-sysext refresh` : actualiza imágenes de extensión del sistema (una combinación de `unmerge` y `merge`)

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-sysext.html

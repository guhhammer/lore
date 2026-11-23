# systemd-mount

Establece y destruye puntos de montaje transitorio o de montaje automático (auto-mount point).

## Usage
- `systemd-mount ruta/al/archivo_o_dispositivo` : monta un sistema de archivos (imagen o dispositivo de bloque (block device)) en `/run/media/system/ETIQUETA` donde ETIQUETA es la etiqueta del sistema de archivos o el nombre del dispositivo si no hay etiqueta
- `systemd-mount ruta/al/archivo_o_dispositivo ruta/al/punto_de_montaje` : monta un sistema de archivos (imagen o dispositivo de bloque) en una ubicación específica
- `systemd-mount --list` : lista todos los dispositivos locales de bloque conocidos con sistemas de archivos que pueden montarse
- `systemd-mount --automount yes ruta/al/archivo_o_dispositivo` : crea un punto de montaje automático que monta el sistema de archivos al momento del primer acceso
- `systemd-mount -u ruta/al/punto_de_montaje_o_dispositivo1 ruta/al/punto_de_montaje_o_dispositivo2 ...` : desmonta uno o más dispositivos
- `systemd-mount -t file_system_type ruta/al/archivo_o_dispositivo ruta/al/punto_de_montaje` : monta un sistema de archivos (dispositivo de imagen o bloque) con un tipo de sistema de archivos específico
- `systemd-mount -o opciones_de_montaje ruta/al/archivo_o_dispositivo ruta/al/punto_de_montaje` : monta un sistema de archivos (imagen o dispositivo de bloque) con opciones adicionales de montaje

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-mount.html

# systemd-dissect

Analiza e interactúa con el sistema de archivos correspondientes a imágenes de disco de sistemas operativos, concretamente con imágenes de disco detectables (DDI).

## Usage
- `systemd-dissect ruta/a/imagen.raw` : muestra información general sobre la imagen del sistema operativo
- `systemd-dissect -m ruta/a/imagen.raw /mnt/imagen` : monta una imagen del sistema operativo
- `systemd-dissect -u /mnt/imagen` : desmonta una imagen del sistema operativo
- `systemd-dissect -l ruta/a/imagen.raw` : muestra una lista de los archivos de una imagen
- `systemd-dissect --attach ruta/a/imagen.raw` : adjunta una imagen del sistema operativo a un dispositivo de bloque de bucle invertido asignado automáticamente e imprime su ruta
- `systemd-dissect --detach ruta/al/dispositivo` : desconecta una imagen del sistema operativo de un dispositivo de bloque de bucle invertido

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-dissect.html

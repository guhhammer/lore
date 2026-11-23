# rkdeveloptool

Flashea, vacía y gestiona el firmware de arranque en dispositivos informáticos basados en Rockchip. Necesitará encender el dispositivo en modo Maskrom/Bootrom antes de conectarlo a través del USB. Algunos subcomandos pueden requerir ser ejecutados como el superusuario.

## Usage
- `rkdeveloptool ld` : lista todos los dispositivos flash conectados basados en Rockchip
- `rkdeveloptool db ruta/al/bootloader.bin` : inicializa el dispositivo forzándolo a descargar e instalar el gestor de arranque desde un archivo
- `rkdeveloptool ul ruta/al/bootloader.bin` : actualiza un gestor de arranque
- `rkdeveloptool wl sector_inicial ruta/a/imagen.img` : escribe una imagen en una partición flash con formato GPT, especificando el sector de almacenamiento inicial (normalmente `0x0`, alias `0`)
- `rkdeveloptool wlx nombre_partición ruta/a/imagen.img` : escribe en la partición flash por su nombre amigable
- `rkdeveloptool rd` : reinicia/repón el dispositivo, sal del modo Maskrom/Bootrom para arrancar en la partición flash seleccionada

## Links
- More information: https://github.com/rockchip-linux/rkdeveloptool

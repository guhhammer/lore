# swapon

Activa dispositivos y archivos para intercambio.

## Usage
- `swapon` : muestra información sobre intercambio
- `sudo swapon ruta/al/archivo` : activa un área de intercambio determinada
- `sudo swapon -a` : activa todas las áreas de intercambio indicadas en `/etc/fstab` excepto las que tienen la opción `noauto`
- `sudo swapon -L etiqueta` : activa una partición de intercambio por su etiqueta

## Links
- More information: https://manned.org/swapon.8

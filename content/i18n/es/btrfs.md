# btrfs

Un sistema de archivos basado en el principio de copia en escritura (COW) para Linux. Algunos subcomandos como `device` tienen su propia documentación de uso.

## Usage
- `sudo btrfs su c ruta/al/subvolumen` : muestra subvolumen
- `sudo btrfs su l ruta/al/punto_de_montaje` : lista subvolúmenes
- `sudo btrfs f df ruta/al/punto_de_montaje` : muestra información sobre el uso del espacio
- `sudo btrfs qu e ruta/al/subvolumen` : habilita cuota
- `sudo btrfs qg s ruta/al/subvolumen` : muestra cuota

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html

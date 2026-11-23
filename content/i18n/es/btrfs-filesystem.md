# btrfs filesystem

Gestiona sistemas de archivos btrfs.

## Usage
- `btrfs filesystem usage ruta/al_montaje_btrfs` : muestra el uso del sistema de archivos (de manera opcional ejecutarlo como root para mostrar información detallada)
- `sudo btrfs filesystem show ruta/al_montaje_btrfs` : muestra el uso por dispositivos individuales
- `sudo btrfs filesystem defragment -v ruta/al/archivo` : desfragmenta un único archivo en un sistema de archivos btrfs (evita mientras un agente de deduplicación esté en ejecución)
- `sudo btrfs filesystem defragment -v -r ruta/al/directorio` : desfragmenta un directorio recursivamente (no cruza los límites de subvolúmenes)
- `sudo btrfs filesystem sync ruta/al_montaje_btrfs` : fuerza la sincronización de bloques de datos no escritos en disco(s)
- `sudo btrfs filesystem du --summarize ruta/al/directorio` : resume el uso del disco para los archivos en un directorio de manera recursiva

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-filesystem.html

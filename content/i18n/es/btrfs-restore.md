# btrfs restore

Intenta recuperar archivos de un sistema de archivos btrfs dañado.

## Usage
- `sudo btrfs rest ruta/al_dispositivo_btrfs ruta/al_directorio_destino` : restaura todos los archivos de un sistema de archivos btrfs a un directorio dado
- `sudo btrfs rest -D ruta/al_dispositivo_btrfs ruta/al_directorio_destino` : lista (sin escribir) los archivos que se van a restaurar de un sistema de archivos btrfs
- `sudo btrfs rest --path-regex regex -c ruta/al_dispositivo_btrfs ruta/al_directorio_destino` : restaura archivos que coincidan con una `regex` dada (insensible a mayúsculas) de un sistema de archivos btrfs (todos los directorios padres de los archivos de destino también deben coincidir)
- `sudo btrfs rest -t bytenr ruta/al_dispositivo_btrfs ruta/al_directorio_destino` : restaura archivos de un sistema de archivos btrfs usando un `bytenr` específico del árbol raíz (ver `btrfs-find-root`)
- `sudo btrfs rest -m -x -S -o ruta/al_dispositivo_btrfs ruta/al_directorio_destino` : restaura archivos de un sistema de archivos btrfs (junto con metadatos, atributos extendidos y enlaces simbólicos), sobrescribiendo archivos en el destino

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-restore.html

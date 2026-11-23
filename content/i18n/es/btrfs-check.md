# btrfs check

Verifica o repara un sistema de archivos btrfs.

## Usage
- `sudo btrfs c ruta/a/la/partición` : verifica un sistema de archivos btrfs
- `sudo btrfs c --repair ruta/a/la/partición` : verifica y repara un sistema de archivos btrfs (peligroso)
- `sudo btrfs c -p ruta/a/la/partición` : muestra el progreso de la verificación
- `sudo btrfs c --check-data-csum ruta/a/la/partición` : verifica la suma de comprobación de cada bloque de datos (si el sistema de archivos es bueno)
- `sudo btrfs c -s n ruta/a/la/partición` : utiliza el superblock `n`-ésimo (`n` puede ser 0, 1 o 2)
- `sudo btrfs c --repair --init-csum-tree ruta/a/la/partición` : reconstruye el árbol de suma de comprobación
- `sudo btrfs c --repair --init-extent-tree ruta/a/la/partición` : reconstruye el árbol de extensiones

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-check.html

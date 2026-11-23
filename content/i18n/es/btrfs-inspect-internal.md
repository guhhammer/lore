# btrfs inspect-internal

Consulta información interna de un sistema de archivos btrfs.

## Usage
- `sudo btrfs i dump-s ruta/a/la_partición` : imprime la información del superbloque
- `sudo btrfs i dump-s -a ruta/a/la_partición` : imprime la información del superbloque y de todas sus copias
- `sudo btrfs i dump-t ruta/a/la_partición` : imprime la información de los metadatos del sistema de archivos
- `sudo btrfs i i n ruta/al_montaje_btrfs` : imprime la lista de archivos en el inodo `n`-ésimo
- `sudo btrfs i lo dirección_lógica ruta/al_montaje_btrfs` : imprime la lista de archivos en una dirección lógica dada
- `sudo btrfs i t ruta/a/la_partición` : imprime estadísticas de los árboles de root, extent, csum y fs

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-inspect-internal.html

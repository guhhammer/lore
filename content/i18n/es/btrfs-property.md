# btrfs property

Obtiene, establece o lista propiedades para un objeto de sistema de archivos BTRFS (archivos, directorios, subvolúmenes, sistemas de archivos o dispositivos).

## Usage
- `sudo btrfs p l ruta/al_objeto_btrfs` : lista las propiedades disponibles (y descripciones) para el objeto btrfs dado
- `sudo btrfs p g ruta/al_objeto_btrfs` : obtiene todas las propiedades para el objeto btrfs dado
- `sudo btrfs p g ruta/al_sistema_de_archivos_btrfs label` : obtiene la propiedad `label` para el sistema de archivos o dispositivo btrfs dado
- `sudo btrfs p g -t subvol|filesystem|inode|device ruta/al_sistema_de_archivos_btrfs` : obtiene todas las propiedades específicas del tipo de objeto para el sistema de archivos o dispositivo btrfs dado
- `sudo btrfs p s ruta/al_inodo_btrfs compression zstd|zlib|lzo|none` : establece la propiedad `compression` para un inodo btrfs dado (ya sea un archivo o un directorio)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-property.html

# btrfs subvolume

Gestiona subvolúmenes e imágenes instantáneas de btrfs.

## Usage
- `sudo btrfs su c ruta/al/nuevo_subvolumen` : crea un nuevo subvolumen vacío
- `sudo btrfs su l ruta/al_sistema_de_archivos_btrfs` : lista todos los subvolúmenes e imágenes instantáneas en el sistema de archivos especificado
- `sudo btrfs su d ruta/al_subvolumen` : elimina un subvolumen
- `sudo btrfs su sn -r ruta/al_subvolumen_origen ruta/al_destino` : crea una imagen instantánea de solo lectura de un subvolumen existente
- `sudo btrfs su sn ruta/al_subvolumen_origen ruta/al_destino` : crea una imagen instantánea de lectura y escritura de un subvolumen existente
- `sudo btrfs su sh ruta/al_subvolumen` : muestra información detallada sobre un subvolumen

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-subvolume.html

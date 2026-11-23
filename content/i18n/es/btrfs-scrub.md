# btrfs scrub

Realiza un scrub en sistemas de archivos btrfs para verificar la integridad de los datos. Se recomienda ejecutar un scrub una vez al mes.

## Usage
- `sudo btrfs sc start ruta/al_montaje_btrfs` : inicia un scrub
- `sudo btrfs sc status ruta/al_montaje_btrfs` : muestra el estado de un scrub en curso o del último completado
- `sudo btrfs sc c ruta/al_montaje_btrfs` : cancela un scrub en curso
- `sudo btrfs sc r ruta/al_montaje_btrfs` : reanuda un scrub previamente cancelado
- `sudo btrfs sc start -B ruta/al_montaje_btrfs` : inicia un scrub, pero espera a que termine antes de salir
- `sudo btrfs sc start -q ruta/al_montaje_btrfs` : inicia un scrub en modo silencioso (no imprime errores ni estadísticas)

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs-scrub.html

# qm disk

Administra imágenes de disco.

## Usage
- `qm di resi 100 disk_name +nG` : añade `n` gigabytes a un disco virtual
- `qm di m 100 destino índice` : mueve un disco virtual
- `qm di m --delete 100 destino índice` : elimina la copia anterior del disco virtual
- `qm di i 100 ruta/al/disco nombre_almacenamiento --format qcow2|raw|vmdk` : importa una imagen de disco VMDK/`.qcow2`/raw utilizando un nombre de almacenamiento específico
- `qm di resc` : vuelve a escanear todos los almacenamientos y actualiza los tamaños de disco y las imágenes de disco no utilizadas
- `qm di resc --dryrun` : realiza una simulación de un nuevo escaneo y no escribe ningún cambio en las configuraciones
- `qm di resc --vmid 100` : especifica una máquina virtual por su ID

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_disk_import

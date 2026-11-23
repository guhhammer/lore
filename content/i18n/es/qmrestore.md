# qmrestore

Restaura copias de seguridad de QemuServer `vzdump`.

## Usage
- `qmrestore ruta/a/vzdump-qemu-100.vma.lzo 100` : restaura la máquina virtual desde un archivo de respaldo dado en el almacenamiento original
- `qmrestore ruta/a/vzdump-qemu-100.vma.lzo 100 --force true` : sobrescribe la máquina virtual existente desde un archivo de respaldo dado en el almacenamiento original
- `qmrestore ruta/a/vzdump-qemu-100.vma.lzo 100 --storage local` : restaura la máquina virtual de un archivo de respaldo dado en el almacenamiento específico
- `qmrestore ruta/a/vzdump-qemu-100.vma.lzo 100 --live-restore true` : inicia la máquina virtual de inmediato desde la copia de seguridad mientras se restaura en el fondo (background) (sólo en el servidor de respaldo Proxmox)

## Links
- More information: https://pve.proxmox.com/pve-docs/qmrestore.1.html

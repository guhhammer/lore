# qm create

Crea o restaura una máquina virtual en el administrador de máquinas virtuales QEMU/KVM.

## Usage
- `qm create 100` : crea una máquina virtual
- `qm create 100 --start 1` : inicia automáticamente la máquina después de su creación
- `qm create 100 --ostype win10` : especifica el tipo de sistema operativo en la máquina virtual
- `qm create 100 --archive ruta/al/archivo_de_respaldo.tar --force 1` : reemplaza una máquina existente (requiere archivarla)
- `qm create 100 --hookscript ruta/al/guión.pl` : especifica un guión (script) a ejecutar automáticamente dependiendo del estado de la máquina virtual

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_create

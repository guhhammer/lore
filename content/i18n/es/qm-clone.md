# qm clone

Crea una copia de la máquina virtual en el Administrador de Máquinas Virtuales QEMU/KVM.

## Usage
- `qm clone 100 101` : copia una máquina virtual
- `qm clone 100 101 --name nombre` : copia una máquina virtual usando un nombre específico
- `qm clone 100 101 --description descripción` : copia una máquina virtual usando una descripción específica
- `qm clone 100 101 --full` : copia una máquina virtual creando una copia completa de todos los discos
- `qm clone 100 101 --full --format qcow2|raw|vmdk` : copia una máquina virtual usando un formato específico para el almacenamiento de archivos (requiere `--full`)
- `qm clone 100 101 --pool nombre_grupo` : copia una máquina virtual y luego la añade a un grupo (pool) específico

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_clone

# qm shutdown

Apaga una máquina virtual del administrador de máquinas virtuales QEMU/KVM.

## Usage
- `qm shu 100` : apaga una máquina virtual
- `qm shu 100 --timeout 10` : apaga una máquina virtual después de esperar por lo menos 10 segundos
- `qm shu 100 --keepActive true` : apaga una máquina virtual y no desactiva los volúmenes de almacenamiento
- `qm shu 100 --skiplock true` : apaga una máquina virtual y omite cualquier bloqueo (solo el root puede usar esta opción)
- `qm shu 100 --forceStop true` : detiene y apaga una máquina virtual

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_shutdown

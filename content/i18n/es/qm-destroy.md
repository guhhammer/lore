# qm destroy

Destruye una máquina virtual del administrador de máquinas virtuales EMU/KVM.

## Usage
- `qm des 100` : destruye una máquina virtual específica
- `qm des 100 --destroy-unreferenced-disks` : destruye todos los discos que no se mencionan explícitamente en la configuración de una máquina virtual específica
- `qm des 100 --purge` : destruye una máquina virtual y la elimina de todos los lugares (inventarios, trabajos de copia de seguridad, manejadores de alta disponibilidad, etc.)
- `sudo qm des 100 --skiplock` : destruye una máquina virtual específica ignorando las cerraduras (locks) y forzando su destrucción

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_destroy

# qm stop

Detiene una máquina virtual.

## Usage
- `qm stop 100` : detiene una máquina virtual inmediatamente
- `qm stop 100 --timeout 10` : detiene una máquina virtual y espera por lo menos 10 segundos
- `qm stop 100 --skiplock true` : detiene una máquina virtual y omite cualquier bloqueo (solo el root puede usar esta opción)
- `qm stop 100 --keepActive true` : detiene una máquina virtual y no desactive los volúmenes de almacenamiento

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_stop

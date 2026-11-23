# qm suspend

Suspende una máquina virtual (MV) en el entorno virtual Proxmox (PVE). Usa las banderas de `--skiplock` y `--skiplockstorage` con precaución, ya que pueden conducir a la corrupción de datos en ciertas situaciones.

## Usage
- `qm su 100 entero` : suspende una máquina virtual por ID
- `qm su 100 entero --skiplock` : omite el chequeo de bloqueo al suspender una MV
- `qm su 100 entero --skiplockstorage` : omite el chequeo de bloqueo por almacenamiento al suspender una MV

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_suspend

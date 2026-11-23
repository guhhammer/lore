# qm migrate

Migra una máquina virtual. Se usa para crear una nueva tarea de migración.

## Usage
- `qm mi 100 destino` : migra una máquina virtual específica
- `qm mi 100 destino --bwlimit 10` : supera el límite de ancho de banda E/S actual con 10 KiB/s
- `qm mi 100 destino --force true` : permite la migración de máquinas virtuales usando dispositivos locales (solo root)
- `qm mi 100 destino --online true` : utiliza la migración en vivo (online) si una máquina virtual está ejecutándose
- `qm mi 100 destino --with-local-disks true` : permite la migración de almacenamiento en vivo para discos locales

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_migrate

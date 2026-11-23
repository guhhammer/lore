# qm config

Muestra la configuración de la máquina virtual con los cambios de configuración pendientes aplicados.

## Usage
- `qm co 100` : muestra la configuración de la máquina virtual
- `qm co --current true 100` : muestra los valores de configuración actuales en lugar de los valores pendientes en la máquina virtual
- `qm co --snapshot nombre_de_la_instantánea 100` : obtiene los valores de configuración de la instantánea (snapshot) dada

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_config

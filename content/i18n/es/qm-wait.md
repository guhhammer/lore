# qm wait

Espera hasta que se detenga la máquina virtual.

## Usage
- `qm w 100` : espera hasta que se detenga la máquina virtual
- `qm w 100 --timeout 10` : espera hasta que la máquina virtual se detenga con un tiempo de espera máximo de 10 segundos
- `qm shu 100 && qm w 100 --timeout 10` : envía una solicitud de apagado, luego espera hasta que la máquina virtual se detenga con un tiempo máximo de espera de 10 segundos

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_wait

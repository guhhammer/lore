# qm sendkey

Envía un evento de teclado del monitor QEMU a una máquina virtual.

## Usage
- `qm sen 100 tecla` : envía el evento de teclado (key) especificado a una máquina virtual específica
- `qm sen --skiplock true 100 tecla` : permite al usuario root enviar el evento clave e ignorar cualquier bloqueo

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_sendkey

# qm guest

Administra un agente invitado de máquina virtual.

## Usage
- `qm g exec-s 100 pid` : imprime el estado de un PID específico
- `qm g p 100 usuario` : establece una contraseña para un usuario específico en una máquina virtual de forma interactiva
- `qm g p 100 usuario --crypted 1` : establece una contraseña ya procesada (hashed) para un usuario específico en una máquina virtual de forma interactiva
- `qm g c 100 fsfreeze-freeze|fsfreeze-status|fsfreeze-thaw|fstrim|get-fsinfo|...` : ejecuta un comando específico de agente huésped en QEMU
- `qm g exec 100 comando primer_argumento segundo_argumento ...` : ejecuta una orden específica a través de un agente huésped
- `qm g exec 100 primer_argumento segundo_argumento ... --synchronous 0` : ejecuta una orden específica a través de un agente huésped asincrónicamente
- `qm g exec 100 primer_argumento segundo_argumento... --timeout 10` : ejecuta una orden específica a través de un agente huésped con un tiempo máximo de 10 segundos
- `qm g exec 100 primer_argumento segundo_argumento ... --pass-stdin 1` : ejecuta una orden específica a través de un agente huésped y reenvía la entrada de `stdin` hasta el fin del archivo (EOF) al agente huésped

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_guest_cmd

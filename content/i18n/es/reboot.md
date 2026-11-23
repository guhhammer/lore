# reboot

Reinicia el sistema.

## Usage
- `reboot` : reinicia inmediatamente
- `reboot -p` : apaga el sistema (igual que `poweroff`)
- `reboot --halt` : detiene (termina todos los procesos y apaga la CPU) el sistema (igual que `halt`)
- `reboot -f` : reinicia inmediatamente sin contactar al administrador del sistema
- `reboot -w` : escribe la entrada wtmp de apagado sin reiniciar el sistema

## Links
- More information: https://manned.org/reboot.8

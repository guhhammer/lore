# modprobe

Aggiunge o rimuove moduli del kernel Linux.

## Usage
- `modprobe -n nome_del_modulo` : fa finta di carica un modulo nel kernel, ma non lo fa veramente
- `sudo modprobe nome_del_modulo` : carica un modulo nel kernel
- `sudo modprobe -r nome_del_modulo` : rimuove un modulo dal kernel
- `sudo modprobe -r --remove-holders nome_del_modulo` : rimuove dal kernel un modulo e quelli che dipendono da quest'ultimo
- `sudo modprobe -D nome_del_modulo` : mostra le dipendenza di un modulo del kernel

## Links
- More information: https://manned.org/modprobe

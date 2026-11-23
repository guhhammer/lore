# dmesg

Escriu els missatges del kernel a la sortida estàndar.

## Usage
- `sudo dmesg` : mostra els missatges del kernel
- `sudo dmesg -l err` : mostra els missatges d'error del kernel
- `sudo dmesg -w` : mostra els missatges del kernel i segueix llegint els nous, similar a `tail -f` (disponible en kernels 3.5.0 i posteriors)
- `sudo dmesg | grep -i memory` : mostra quanta memòria física hi ha disponible en el sistema
- `sudo dmesg | less` : mostra tots els missatges del kernel, pàgina a pàgina
- `sudo dmesg -T` : mostra els missatges del kernel amb una estampa temporal (disponible en kernels 3.5.0 i posteriors)
- `sudo dmesg -H` : mostra els missatges del kernel de forma llegible per humans (disponible en kernels 3.5.0 i posteriors)
- `sudo dmesg -L` : pinta la sortida (disponible en kernels 3.5.0 i posteriors)

## Links
- More information: https://manned.org/dmesg

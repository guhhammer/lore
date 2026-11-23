# eselect kernel

Un modulo `eselect` per la gestione del collegamento simbolico `/usr/src/linux`.

## Usage
- `eselect kernel list` : elenca i target del collegamento simbolico del kernel disponibili con i loro numeri
- `eselect kernel set nome|numero` : imposta il collegamento simbolico `/usr/src/linux` per nome o numero dal comando `list`
- `eselect kernel show` : mostra a cosa punta attualmente il collegamento simbolico del kernel
- `eselect kernel update` : imposta il collegamento simbolico del kernel al kernel attualmente in esecuzione

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Kernel

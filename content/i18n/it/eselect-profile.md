# eselect profile

Un modulo `eselect` per la gestione del collegamento simbolico `/etc/portage/make.profile`, che imposta il profilo di sistema.

## Usage
- `eselect profile list` : elenca i target del collegamento simbolico del profilo disponibili con i loro numeri
- `eselect profile set nome|numero` : imposta il collegamento simbolico `/etc/portage/make.profile` per nome o numero dal comando `list`
- `eselect profile show` : mostra il profilo di sistema corrente

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#Profile

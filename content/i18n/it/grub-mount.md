# grub-mount

Monta un filesystem o un'immagine di filesystem in sola lettura usando i driver GRUB.

## Usage
- `grub-mount /dev/sdXY /mnt` : monta un dispositivo a blocchi o un'immagine di filesystem su un punto di mount
- `grub-mount -r 2 disk.img /mnt` : monta la seconda partizione di un'immagine disco completa, `-r` specifica il numero di partizione nell'immagine
- `grub-mount -C /dev/sdXY /mnt` : monta un dispositivo crittografato e richiede una passphrase
- `grub-mount -K /path/to/zfs.key /dev/sdX /mnt` : carica una chiave di crittografia ZFS da un file
- `grub-mount -d string image /mnt` : mostra output di debug per una categoria corrispondente
- `grub-mount -v image /mnt` : abilita output verboso
- `grub-mount -?` : mostra l'aiuto
- `grub-mount --version` : mostra la versione

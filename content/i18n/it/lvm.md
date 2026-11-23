# lvm

Gestisci volumi fisici, gruppi di volumi e volumi logici utilizzando la shell interattiva Logical Volume Manager (LVM).

## Usage
- `sudo lvm` : avvia la shell interattiva Logical Volume Manager
- `sudo lvm pvcreate /dev/sdXY` : inizializza un'unità o partizione da utilizzare come volume fisico
- `sudo lvm pvdisplay` : visualizza informazioni sui volumi fisici
- `sudo lvm vgcreate vg1 /dev/sdXY` : crea un gruppo di volumi chiamato vg1 dal volume fisico su `/dev/sdXY`
- `sudo lvm vgdisplay` : visualizza informazioni sui gruppi di volumi
- `sudo lvm lvcreate -L 10G vg1` : crea un volume logico con dimensione 10G dal gruppo di volumi vg1
- `sudo lvm lvdisplay` : visualizza informazioni sui volumi logici
- `lvm help comando` : visualizza la guida per un comando specifico

## Links
- More information: https://manned.org/lvm

# lvm

Administración de volúmenes fisicos, grupos de volúmenes, y volúmenes lógicos mediante la terminal interactiva de Logical Volume Manager (LVM).

## Usage
- `sudo lvm` : inicia la terminal interactiva Logical Volume Manager
- `sudo lvm pvcreate /dev/sdXY` : inicializa un disco o partición para ser utilizado como volumen físico
- `sudo lvm pvdisplay` : imprime información sobre volúmenes físicos
- `sudo lvm vgcreate vg1 /dev/sdXY` : crea un grupo de volumen llamado vg1 a partir del volumen físico en `/dev/sdXY`
- `sudo lvm vgdisplay` : imprime información sobre grupos de volumen
- `sudo lvm lvcreate -L 10G vg1` : crea un volumen lógico con un tamaño de 10G a partir del grupo de volumen vg1
- `sudo lvm lvdisplay` : imprime información sobre volúmenes lógicos
- `lvm help comando` : imprime ayuda para un comando específico

## Links
- More information: https://manned.org/lvm

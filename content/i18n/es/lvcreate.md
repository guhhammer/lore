# lvcreate

Crea un volumen lógico en un grupo de volumen existente. Un grupo de volumen es una colección de volúmenes lógicos y físicos.

## Usage
- `sudo lvcreate -L 10G vg1` : crea un volumen lógico de 10 GB en el grupo de volumen vg1
- `sudo lvcreate -L 1500 -n mylv vg1` : crea un volumen lógico lineal de 1500 MB llamado mylv en el grupo de volumen vg1
- `sudo lvcreate -l 60%VG -n mylv vg1` : crea un volumen lógico llamado mylv que utiliza el 60% del espacio total del grupo de volumen vg1
- `sudo lvcreate -l 100%FREE -n mylv vg1` : crea un volumen lógico llamado mylv que utiliza todo el espacio sin asignar del grupo de volumen vg1

## Links
- More information: https://manned.org/lvcreate

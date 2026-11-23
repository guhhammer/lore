# mkfs

Crea un sistema de archivos de Linux en una partición de disco duro. Nota: Este comando está obsoleto, use comandos específicos de cada sistema de archivos como `mkfs.ext4` en su lugar.

## Usage
- `sudo mkfs /dev/sdXY` : crea un sistema de archivos ext2 de Linux en una partición
- `sudo mkfs -t ext4 /dev/sdXY` : crea un sistema de archivos de un tipo específico
- `sudo mkfs -c -t ntfs /dev/sdXY` : crea un sistema de archivos de un tipo específico y busca bloques defectuosos

## Links
- More information: https://manned.org/mkfs

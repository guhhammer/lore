# wipefs

Borra firmas de sistemas de archivos, RAID o tablas de particiones de un dispositivo.

## Usage
- `sudo wipefs /dev/sdX` : muestra las firmas del dispositivo especificado
- `sudo wipefs -a /dev/sdX` : borra todos los tipos de firma disponibles de un dispositivo específico sin recursión en las particiones
- `sudo wipefs -a /dev/sdX*` : borra todos los tipos de firma disponibles del dispositivo y de sus particiones usando un patrón `glob`
- `sudo wipefs -a -n /dev/sdX` : simula el borrado de todos los tipos de firma de un dispositivo sin modificarlo realmente
- `sudo wipefs -a -f /dev/sdX` : fuerza el borrado, incluso si el sistema de archivos está montado

## Links
- More information: https://manned.org/wipefs

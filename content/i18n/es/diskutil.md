# diskutil

Utilidad para gestionar discos y volúmenes locales.

## Usage
- `diskutil list` : lista todos los discos, particiones y volúmenes montados actualmente disponibles
- `diskutil repairVolume /dev/disk_device` : repara las estructuras de datos del sistema de archivos de un volumen
- `diskutil unmountDisk /dev/disk_device` : desmonta un volumen
- `diskutil eject /dev/disk_device1` : expulsa un CD/DVD (primero lo desmonta)

## Links
- More information: https://keith.github.io/xcode-man-pages/diskutil.8.html

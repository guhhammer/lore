# mkfs.erofs

Crea un sistema de archivos EROFS en una imagen.

## Usage
- `mkfs.erofs image.erofs root/` : crea un sistema de archivos EROFS basado en el directorio raíz
- `mkfs.erofs -U uuid image.erofs root/` : crea una imagen EROFS con un UUID específico
- `mkfs.erofs -zlz4hc image.erofs root/` : crea una imagen EROFS comprimida
- `mkfs.erofs --all-root image.erofs root/` : crea una imagen EROFS en la que todos los archivos pertenezcan a root

## Links
- More information: https://manned.org/mkfs.erofs

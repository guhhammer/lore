# umount

Desvincula un sistema de archivos de su punto de montaje, haciendo que deje de ser accesible. Un sistema de archivos no se puede desmontar mientras está en uso.

## Usage
- `sudo umount ruta/al/archivo_de_dispositivo` : desmonta un sistema de archivos indicando la ruta del origen desde el que está montado
- `sudo umount ruta/al/directorio_montado` : desmonta un sistema de archivos indicando la ruta del destino donde está montado
- `sudo umount -r ruta/al/directorio_montado` : si el desmontaje falla, intenta volver a montar el sistema de archivos en modo de solo lectura
- `sudo umount -R ruta/al/directorio_montado` : desmonta recursivamente cada directorio indicado
- `sudo umount -a` : desmonta todos los sistemas de archivos montados (excepto el sistema de archivos `proc`)
- `sudo umount -l ruta/al/directorio_montado` : separa un sistema de archivos de la jerarquía de sistemas de archivos y lo desmonta en cuanto no haya referencias al mismo

## Links
- More information: https://manned.org/umount.8

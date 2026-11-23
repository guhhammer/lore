# modprobe

Añade o elimina módulos del núcleo Linux.

## Usage
- `modprobe -n nombre_del_módulo` : finge cargar un módulo en el kernel, pero no lo hace realmente
- `sudo modprobe nombre_del_módulo` : carga un módulo en el kernel
- `sudo modprobe -r nombre_del_módulo` : elimina un módulo del núcleo
- `sudo modprobe -r --remove-holders nombre_del_módulo` : elimina un módulo y los que dependen de él desde el núcleo
- `sudo modprobe -D nombre_del_módulo` : muestra las dependencias de un módulo del kernel

## Links
- More information: https://manned.org/modprobe

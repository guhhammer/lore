# virt-qemu-run

Herramienta experimental para ejecutar una Guest VM QEMU independiente de `libvirtd`.

## Usage
- `virt-qemu-run ruta/a/guest.xml` : ejecuta una máquina virtual QEMU
- `virt-qemu-run -r ruta/al/directorio ruta/a/guest.xml` : ejecuta una máquina virtual QEMU y almacena el estado en un directorio específico
- `virt-qemu-run -v ruta/a/guest.xml` : ejecuta una máquina virtual QEMU y muestra información detallada sobre el inicio
- `virt-qemu-run -h` : muestra la ayuda

## Links
- More information: https://libvirt.org/manpages/virt-qemu-run.html

# systool

Vea información de dispositivos del sistema por bus y clases. Este comando es parte del paquete `sysfs`.

## Usage
- `systool -b bus -v` : lista todos los atributos de los dispositivos de un bus (ej. `pci`, `usb`). Vea todos los buses usando `ls /sys/bus`
- `systool -c clase -v` : lista todos los atributos de una clase de dispositivos (ej. `drm`, `block`). Vea todas las clases usando `ls /sys/class`
- `systool -b bus -D` : muestra solo los controladores de un bus (ej. `pci`, `usb`)

## Links
- More information: https://manned.org/systool

# lsusb

Muestra información sobre los buses USB y los dispositivos conectados a ellos.

## Usage
- `lsusb` : muestra todos los dispositivos USB disponibles
- `lsusb -t` : lista la jerarquía USB como un árbol
- `sudo lsusb -v` : muestra información detallada sobre los dispositivos USB
- `sudo lsusb -v -s bus:número de dispositivo` : muestra información detallada sobre un dispositivo USB
- `lsusb -d vendedor:producto` : muestra sólo los dispositivos con un ID de proveedor y producto determinados

## Links
- More information: https://manned.org/lsusb

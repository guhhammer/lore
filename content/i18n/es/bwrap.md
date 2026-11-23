# bwrap

Ejecuta programas en un sandbox ligero.

## Usage
- `bwrap --ro-bind / / /bin/bash` : ejecuta un programa en un entorno de sólo lectura
- `bwrap --dev-bind /dev /dev --proc /proc --ro-bind / / --tmpfs /tmp /bin/bash` : da al entorno acceso a dispositivos, información de procesos y crea un `tmpfs` para el mismo

## Links
- More information: https://manned.org/bwrap

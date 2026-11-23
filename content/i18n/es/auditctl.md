# auditctl

Utilidad para controlar el comportamiento, obtener el estado y gestionar las reglas del Sistema de Auditoría de Linux.

## Usage
- `sudo auditctl -s` : muestra el estado del sistema de auditoría
- `sudo auditctl -l` : muestra todas las reglas de auditoría cargadas actualmente
- `sudo auditctl -D` : elimina todas las reglas de auditoría
- `sudo auditctl -e 1|0` : habilita/deshabilita el sistema de auditoría
- `sudo auditctl -a always,exit -F arch=b64 -F path=/ruta/al/archivo -F perm=wa` : vigila un archivo en busca de cambios
- `sudo auditctl -a always,exit -F arch=b64 -F dir=/ruta/al/directorio/ -F perm=wa` : busca cambios en un directorio de forma recursiva
- `auditctl -h` : muestra la ayuda

## Links
- More information: https://manned.org/auditctl

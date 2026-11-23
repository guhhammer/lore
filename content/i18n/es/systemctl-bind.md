# systemctl bind

Monta temporalmente un archivo o directorio del host en el espacio de nombres de montaje de una unidad.

## Usage
- `systemctl bind unidad /ruta/al/directorio_host` : monta una ruta del host en la misma ubicación dentro de la unidad
- `systemctl bind unidad /ruta/al/directorio_host /ruta/al/directorio_unidad` : monta una ruta del host en una ubicación diferente dentro de la unidad
- `systemctl bind unidad /ruta/al/directorio_host --read-only` : monta una ruta como solo lectura dentro de la unidad
- `systemctl bind unidad /ruta/al/directorio_host /ruta/al/directorio_unidad --mkdir` : crea la ruta de destino dentro de la unidad antes de montar

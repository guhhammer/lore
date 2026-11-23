# systemctl is-active

Verifica si una o más unidades de systemd están activas.

## Usage
- `systemctl is-active unidad` : verifica si una unidad está activa
- `systemctl is-active unidad1 unidad2 ...` : verifica si múltiples unidades están activas
- `systemctl is-active unidad -q` : verifica si una unidad está activa sin imprimir el estado en `stdout`
- `systemctl is-active unidad --user` : verifica si una unidad de usuario está activa

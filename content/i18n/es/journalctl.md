# journalctl

Consulta el registro systemd.

## Usage
- `journalctl -b -p 3` : muestra todos los mensajes con nivel de prioridad 3 (errores) de este boot
- `journalctl --vacuum-time 2d` : elimina los registros diarios con más de 2 días de antigüedad
- `journalctl -n N -f` : muestra solo las últimas N líneas y sigue los mensajes nuevos (como `tail -f` de un syslog tradicional)
- `journalctl -u unidad` : muestra todos los mensajes de una unidad específica
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID unidad)` : muestra los registros de una unidad determinada desde la última vez que se inició
- `journalctl -S now|today|yesterday|tomorrow -U "YYYY-MM-DD HH:MM:SS"` : filtra mensajes dentro de un intervalo de tiempo (marca de tiempo o marcadores de posición como "ayer")
- `journalctl _PID=pid` : muestra todos los mensajes de un proceso específico
- `journalctl ruta/al/ejecutable` : muestra todos los mensajes de un ejecutable específico

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html

# systemctl-halt

Apaga y detiene el sistema (detiene el kernel del SO pero mantiene el hardware encendido).

## Usage
- `systemctl halt` : detiene el sistema
- `systemctl halt -f` : detiene el sistema inmediatamente sin solicitar a los servicios que se detengan elegantemente
- `systemctl halt -f --no-wall` : detiene el sistema inmediatamente sin enviar notificaciones a los usuarios conectados
- `systemctl halt -ff` : detiene el sistema inmediatamente sin terminar procesos o desmontar sistemas de archivos (peligroso, puede causar pérdida de datos)
- `systemctl halt --when 23:00` : programa una detención a una hora específica (ej., 23:00)
- `systemctl halt --when +2h` : programa una detención después de cierta duración (ej., 2 horas)
- `systemctl halt --when cancel` : cancela una detención programada

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html#halt

# service

Gestiona los servicios mediante la ejecución de scripts init. Se debe omitir la ruta completa del script (se asume `/etc/init.d/`).

## Usage
- `service --status-all` : lista el nombre y el estado de todos los servicios
- `service nombre_de_servicio start|stop|restart|reload` : inicia/Para/Reinicia/Recarga servicio (start/stop debería estar siempre disponible)
- `service nombre_de_servicio --full-restart` : hace un reinicio completo (ejecuta el script dos veces con start y stop)
- `service nombre_de_servicio status` : muestra el estado actual de un servicio

## Links
- More information: https://manned.org/service

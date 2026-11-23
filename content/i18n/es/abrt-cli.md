# abrt-cli

Herramienta automática de reporte de errores para sistemas basados en Fedora. Se utiliza para detectar, analizar y notificar fallos de aplicaciones.

## Usage
- `abrt-cli list` : lista los problemas detectados
- `abrt-cli info problema_id` : muestra los detalles de un problema específico
- `abrt-cli remove problema_id` : elimina un informe de fallo
- `abrt-cli report problema_id` : informa de un problema al gestor de errores configurado (por ejemplo, Bugzilla)
- `abrt-watch-log -F cadena_error /var/log/myapp.log notify-send "Fallo detectado"` : monitorea un archivo de registro y activa un programa cuando se encuentra una coincidencia
- `abrt-cli report -a problema_id` : genera un informe para depurar manualmente

## Links
- More information: https://abrt.readthedocs.io/en/latest/usage.html

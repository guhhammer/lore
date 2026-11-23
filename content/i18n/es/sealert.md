# sealert

Analiza y explica los mensajes de denegación AVC de SELinux. Forma parte del paquete `setroubleshoot-server`.

## Usage
- `sudo sealert -a /var/log/audit/audit.log` : analiza todas las denegaciones recientes de SELinux
- `sudo sealert -l id_alerta` : analiza un ID de alerta específico de los registros del sistema
- `sudo sealert -b` : muestra un resumen de las alertas recientes de SELinux
- `sudo tail -f /var/log/audit/audit.log | sealert -l -` : supervisa el registro de auditoría en tiempo real en busca de nuevas alertas

## Links
- More information: https://manned.org/sealert

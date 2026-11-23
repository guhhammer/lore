# xfreerdp

Implementación del protocolo del Free Remote Desktop.

## Usage
- `xfreerdp /u:usuario /p:contraseña /v:dirección_ip` : conéctate a un servidor FreeRDP
- `xfreerdp /u:usuario /p:contraseña /v:dirección_ip /sound:sys:alsa` : conéctate a un servidor FreeRDP y activa la redirección de la salida de audio mediante el dispositivo `sys:alsa`
- `xfreerdp /v:dirección_ip /u:usuario /p:contraseña /dynamic-resolution` : conéctate a un servidor FreeRDP con resolución dinámica
- `xfreerdp /v:dirección_ip /u:usuario /p:contraseña +clipboard` : conéctate a un servidor FreeRDP con redirección del portapapeles
- `xfreerdp /v:dirección_ip /u:usuario /p:contraseña /cert:ignore` : conéctate a un servidor FreeRDP ignorando cualquier comprobación de certificado
- `xfreerdp /v:dirección_ip /u:usuario /p:contraseña /drive:ruta/al/directorio,nombre_compartido` : conéctate a un servidor FreeRDP con un directorio compartido

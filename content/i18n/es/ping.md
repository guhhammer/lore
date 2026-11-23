# ping

Envía paquetes ICMP ECHO_REQUEST (pings) a equipos (hosts) de la red.

## Usage
- `ping host` : envía pings a un host
- `ping -c numero host` : envía un número determinado de pings a un host
- `ping -i segundos host` : envía pings a un host especificando el intervalo de tiempo entre peticiones (por defecto 1 segundo)
- `ping -n host` : envía pings a un host sin intentar resolver nombres simbólicos de direcciones
- `ping -a host` : envía pings a un host y emite un sonido cuando un paquete es recibido (si la terminal lo soporta)
- `ping -O host` : muestra también un mensaje si no se recibió respuesta
- `ping -c cantidad -W segundos -w segundos host` : envía una cantidad específica de pings (`-c`), con un tiempo límite (`-W`) para cada respuesta, y un tiempo máximo total (`-w`) para la ejecución del ping completo

## Links
- More information: https://manned.org/ping

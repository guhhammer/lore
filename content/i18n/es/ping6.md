# ping6

Envía paquetes ICMP ECHO_REQUEST (pings) a hosts de la red usando direcciones IPv6.

## Usage
- `ping6 host` : envía pings a un host
- `ping6 -c numero host` : envía un número específico de pings a un host
- `ping6 -i segundos host` : envía pings a un host, especificando el intervalo de tiempo entre peticiones (por defecto es 1 segundo)
- `ping6 -n host` : envía pings a un host sin intentar resolver nombres simbólicos de direcciones
- `ping6 -a host` : envía pings a un host y emite un sonido cuando un paquete es recibido (si la terminal lo soporta)

## Links
- More information: https://manned.org/ping6

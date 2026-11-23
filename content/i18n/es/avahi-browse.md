# avahi-browse

Muestra los servicios y hosts expuestos en la red local a través de mDNS/DNS-SD. Avahi es compatible con Bonjour (Zeroconf) encontrado en dispositivos Apple.

## Usage
- `avahi-browse -a -r -l` : lista los servicios disponibles en la red local junto con sus direcciones y puertos, ignorando los de la máquina local
- `avahi-browse -a -t -p` : lista rápidamente los servicios en la red local en formato SSV para scripts
- `avahi-browse -D` : lista los dominios en el vecindario
- `avahi-browse -a --domain=dominio` : limita la búsqueda a un dominio específico

## Links
- More information: https://manned.org/avahi-browse

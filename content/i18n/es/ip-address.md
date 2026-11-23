# ip address

Subcomando de manejo de direcciones IP.

## Usage
- `ip a` : lista las interfaces de red y sus direcciones IP asociadas
- `ip a s up` : filtra mostrando solo las interfaces de red activas
- `ip a s eth0` : muestra información sobre una interfaz de red específica
- `sudo ip a a dirección_ip dev eth0` : añade una dirección IP a una interfaz de red
- `sudo ip a d dirección_ip dev eth0` : elimina una dirección IP de una interfaz de red
- `sudo ip a f eth0 scope global|host|link` : elimina todas las direcciones IP en un alcance dado de una interfaz de red

## Links
- More information: https://manned.org/ip-address

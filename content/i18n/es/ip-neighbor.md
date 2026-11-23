# ip neighbor

Subcomando de gestión de tablas IP de vecinos/ARP.

## Usage
- `ip n` : muestra las entradas de la tabla de vecinos/ARP
- `sudo ip n f dev eth0` : elimina entradas en la tabla de vecinos del dispositivo `eth0`
- `ip n g lookup_ip dev eth0` : realiza una búsqueda de vecinos y devuelve una entrada de vecinos
- `sudo ip n add|delete dirección_ip lladdr mac_address dev eth0 nud reachable` : agrega o elimina una entrada ARP a los vecinos IP de `eth0`
- `sudo ip n change|replace dirección_ip lladdr nueva_mac_address dev eth0` : cambia o reemplaza una entrada ARP para la dirección IP vecina a `eth0`

## Links
- More information: https://manned.org/ip-neighbour

# arping

Descubre y sondea hosts en una red utilizando el protocolo ARP. Útil para el descubrimiento de direcciones MAC.

## Usage
- `sudo arping host_ip` : hace ping a un host mediante paquetes de petición ARP
- `sudo arping -I interfaz host_ip` : hace ping a un host en una interfaz específica
- `sudo arping -f host_ip` : hace ping a un host y detenerse en la primera respuesta
- `sudo arping -c cuenta host_ip` : hace ping a un host un determinado número de veces
- `sudo arping -U ip_a_retransmitir` : emite paquetes de solicitud ARP para actualizar las cachés ARP de los vecinos
- `sudo arping -D -w 3 ip_a_verificar` : detecta direcciones IP duplicadas en la red enviando peticiones ARP con un tiempo de espera de 3 segundos

## Links
- More information: https://manned.org/arping

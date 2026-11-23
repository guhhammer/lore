# ip route get

Obtiene una única ruta a un destino e imprime su contenido exactamente como el kernel lo ve.

## Usage
- `ip r g 1.1.1.1` : imprime ruta a un destino
- `ip r g destino from origen` : imprime la ruta a un destino desde una dirección de origen específica
- `ip r g destino iif eth0` : imprime la ruta a un destino para los paquetes que llegan a una interfaz específica
- `ip r g destino oif eth1` : imprime la ruta a un destino forzando la salida a través de una interfaz específica
- `ip r g destino tos 0x10` : imprime la ruta a un destino con un tipo específico de servicio (ToS)
- `ip r g destino vrf myvrf` : imprime la ruta a un destino utilizando una instancia VRF específica (Virtual Routing and Forwarding - Enrutamiento y Reenvío Virtual)

## Links
- More information: https://manned.org/ip-route

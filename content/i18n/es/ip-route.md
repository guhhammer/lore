# ip route

Subcomando de gestión de tablas de enrutamiento IP.

## Usage
- `ip r` : muestra la tabla de enrutamiento
- `sudo ip r a default via ip_de_gateway` : agrega una ruta predeterminada usando reenvío de puerta de enlace (gateway)
- `sudo ip r a default dev eth0` : añade una ruta predeterminada utilizando `eth0`
- `sudo ip r a ip_destino via ip_de_gateway dev eth0` : añade una ruta estática
- `sudo ip r d ip_destino dev eth0` : elimina una ruta estática
- `sudo ip r change|replace ip_destino via ip_de_gateway dev eth0` : cambia o reemplaza una ruta estática
- `ip r g ip_destino` : muestra qué ruta será utilizada por el kernel para llegar a una dirección IP

## Links
- More information: https://manned.org/ip-route

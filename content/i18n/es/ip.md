# ip

Muestra/manipula enrutamientos, dispositivos, políticas de enrutamiento y túneles. Algunos subcomandados como `address` tienen su propia documentación de uso.

## Usage
- `ip a` : lista las interfaces con información detallada
- `ip -br a` : lista las interfaces con información breve de capa de red
- `ip -br l` : lista las interfaces con información breve dada una capa de enlace
- `ip r` : muestra la tabla de enrutamiento
- `ip n` : muestra vecinos (tabla ARP)
- `sudo ip l s interfaz up|down` : establece una interfaz arriba/abajo (up/down). Usa inglés
- `sudo ip a add|delete ip_address/mask dev interfaz` : agrega/borra una dirección IP de una interfaz
- `sudo ip r a default via ip_address dev interfaz` : agrega una ruta predeterminada

## Links
- More information: https://manned.org/ip.8

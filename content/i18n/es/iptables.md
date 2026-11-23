# iptables

Configura tablas, cadenas y reglas del firewall IPv4 del kernel Linux. Utiliza `ip6tables` para establecer reglas para el tráfico IPv6.

## Usage
- `sudo iptables -vnL --line-numbers` : muestra cadenas, reglas, contadores de paquetes/bytes y números de línea para la tabla de filtros (filter table)
- `sudo iptables -P cadena regla` : establece la política de cadena de la regla
- `sudo iptables -A cadena -s ip_address -j regla` : añade una regla a la política de cadena para la IP
- `sudo iptables -A cadena -s ip_address -p tcp|udp|icmp|... --dport puerto -j regla` : añade una regla a la política de cadena para la IP teniendo en cuenta el protocolo y el puerto
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : agrega una regla NAT para traducir todo el tráfico desde la subred `192.168.0.0/24` a la IP pública del host
- `sudo iptables -D cadena número_de_línea_de_regla` : borra la regla de la cadena

## Links
- More information: https://manned.org/iptables

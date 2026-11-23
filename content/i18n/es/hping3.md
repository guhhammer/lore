# hping3

Utilidad de ping avanzada que soporta protocolos TCP, UDP y raw IP. Mejor correrla con privilegios elevados.

## Usage
- `hping3 -1 -c 4 ip_o_nombre_de_servidor` : ping a un destino con 4 solicitudes ping ICMP
- `hping3 -2 -p 80 -S ip_o_nombre_de_servidor` : ping a una dirección IP sobre UDP en el puerto 80
- `hping3 -V -S -p 80 -s 5090 ip_o_nombre_de_servidor` : escanea el puerto TCP 80, haciéndolo desde el puerto de origen local 5090
- `hping3 -T -V -S -p 80 ip_o_nombre_de_servidor` : traceroute utilizando un escaneado TCP a un puerto de destino específico
- `hping3 -8 80,3000,9000 -S ip_o_nombre_de_servidor` : escanea un conjunto de puertos TCP en una dirección IP específica
- `hping3 -c 2 -V -p 80 -A ip_o_nombre_de_servidor` : realiza un escaneado TCP ACK para comprobar si un equipo dado está vivo
- `hping3 --flood -p 80 -S ip_o_nombre_de_servidor` : realiza una prueba de carga en el puerto 80

## Links
- More information: https://manned.org/hping3

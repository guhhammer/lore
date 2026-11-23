# arptables

Gestiona las reglas de filtrado ARP usando el backend `nftables`. Parte de la suite `xtables-nft` para filtrado de paquetes ARP.

## Usage
- `sudo arptables -L` : lista todas las reglas ARP en la tabla de filtrado
- `sudo arptables -A INPUT -s 192.168.0.1 -j DROP` : añade una regla para descartar paquetes ARP de una dirección IP específica
- `sudo arptables -D INPUT número_de_regla` : elimina una regla específica de la cadena INPUT por su número de regla
- `sudo arptables -F` : borra todas las reglas de la tabla de filtros
- `sudo arptables -P OUTPUT ACCEPT` : establece la política predeterminada de la cadena OUTPUT en ACCEPT
- `sudo arptables-save >ruta/al/archivo` : guarda las reglas ARP actuales en un archivo

## Links
- More information: https://manned.org/arptables

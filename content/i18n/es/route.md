# route

Manipula manualmente las tablas de enrutamiento. Necesita ser root.

## Usage
- `sudo route add "dirección_ip_destino" "dirección_puerta"` : añade una ruta a un destino a través de una puerta de enlace
- `sudo route add "dirección_ip_subred/24" "dirección_puerta"` : añade una ruta a una subred /24 a través de una puerta de enlace
- `sudo route -t add "dirección_ip_destino/24" "dirección_puerta"` : ejecuta en modo de prueba (no hace nada, sólo imprime)
- `sudo route flush` : elimina todas las rutas
- `sudo route delete "dirección_ip_destino/24"` : elimina una ruta específica
- `sudo route get "destino"` : busca y muestra la ruta de un destino (nombre de host o dirección IP)

## Links
- More information: https://keith.github.io/xcode-man-pages/route.8.html

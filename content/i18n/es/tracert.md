# tracert

Recibe información sobre cada paso en la ruta entre tu PC y el destino.

## Usage
- `tracert ip_address` : rastrea una ruta
- `tracert /d ip_address` : evita que `tracert` resuelva direcciones IP a nombres de host
- `tracert /4 ip_address` : obliga a que `tracert` use solo IPv4
- `tracert /6 ip_address` : obliga a que `tracert` use solo IPv6
- `tracert /h max_saltos ip_address` : especifica el número máximo de saltos en la búsqueda del destino
- `tracert /?` : muestra la ayuda

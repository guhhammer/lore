# ip route list

Subcomando de visualización para la gestión de la tabla de rutas IP.

## Usage
- `ip r l` : muestra la tabla de enrutamiento `main`
- `ip r l t main|254` : muestra la tabla de enrutamiento principal (igual que el primer ejemplo)
- `ip r l t local|255` : muestra la tabla de enrutamiento local
- `ip r l t all|unspec|0` : muestra todas las tablas de enrutamiento
- `ip r l dev ethX` : lista solo las rutas de un dispositivo determinado
- `ip r l s link` : lista las rutas dentro de un ámbito determinado
- `ip r l c` : muestra la caché de enrutamiento
- `ip -6|-4 r` : muestra solo rutas IPv6 o IPv4

## Links
- More information: https://manned.org/ip-route

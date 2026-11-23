# resolvectl

Resuelve nombres de dominio, direcciones IPv4 e IPv6, registros de recursos DNS y servicios.

## Usage
- `resolvectl` : muestra la configuración de DNS
- `resolvectl query dominio1 dominio2 ...` : resuelve las direcciones IPv4 e IPv6 de uno o más dominios
- `resolvectl query dirección_IP` : obtiene el dominio de una dirección IP especificada
- `resolvectl flush-caches` : vacía todas las cachés DNS locales
- `resolvectl statistics` : muestra estadísticas de DNS (transacciones, caché y veredictos DNSSEC)
- `resolvectl --legend no -t MX query dominio` : obtiene un registro MX de un dominio
- `resolvectl service _servicio._protocolo nombre` : resuelve un registro SRV, por ejemplo `_xmpp-server._tcp gmail.com`
- `resolvectl tlsa tcp dominio:443` : obtiene una clave TLS

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html

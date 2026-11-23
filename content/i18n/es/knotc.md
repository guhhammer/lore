# knotc

Controla el servidor DNS knot.

## Usage
- `knotc zone-begin zona` : comienza a editar una zona
- `knotc zone-set zona subdominio 3600 A dirección_ip` : establece un registro A con TTL de 3600
- `knotc zone-commit zona` : finaliza la edición de la zona
- `knotc zone-read zona` : obtén los datos de la zona actual
- `knotc conf-read server` : obtén la configuración actual del servidor

## Links
- More information: https://www.knot-dns.cz/docs/latest/html/man_knotc.html

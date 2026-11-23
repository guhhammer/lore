# ip rule

Gestión de la base de datos de políticas de enrutamiento IP.

## Usage
- `ip ru` : muestra la política de enrutamiento
- `sudo ip ru a from all lookup tabla_id` : crea una nueva regla de enrutamiento genérica con una prioridad mayor que `main`
- `sudo ip ru a from 192.168.178.2/32 lookup table_id` : añade una nueva regla basada en las direcciones de origen de los paquetes
- `sudo ip ru a to 192.168.178.2/32 lookup table_id` : añade una nueva regla basada en las direcciones de destino de los paquetes
- `sudo ip ru d from 192.168.178.2/32` : elimina una regla basada en las direcciones de origen de los paquetes
- `sudo ip ru f` : elimina todas las reglas de enrutamiento
- `ip ru s >ruta/a/ip_reglas.dat` : guarda todas las reglas en un archivo
- `sudo ip <ruta/a/ip_reglas.dat ru r` : restaura todas las reglas desde un archivo

## Links
- More information: https://manned.org/ip-rule

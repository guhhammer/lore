# adig

Muestra la información recibida de los servidores del Sistema de Nombres de Dominio (DNS).

## Usage
- `adig example.com` : muestra el registro A (predeterminado) desde DNS para el(los) nombre(s) de host
- `adig -d example.com` : muestra salida adicional de depuración
- `adig -s 1.2.3.4 example.com` : conecta a un servidor DNS especificado
- `adig -T puerto example.com` : usa un puerto TCP específico para conectarse a un servidor DNS
- `adig -U puerto example.com` : usa un puerto UDP específico para conectarse a un servidor DNS

## Links
- More information: https://manned.org/adig

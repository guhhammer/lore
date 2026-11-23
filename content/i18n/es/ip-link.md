# ip link

Gestiona interfaces de red.

## Usage
- `ip l` : muestra información sobre todas las interfaces de red
- `ip l sh ethN` : muestra información sobre una interfaz de red específica
- `sudo ip l s ethN up|down` : establece una interfaz de red arriba (up) o abajo (down). Usa inglés
- `sudo ip l s ethN al "LAN Interface"` : establece un nombre significativo a una interfaz de red
- `sudo ip l s ethN a ff:ff:ff:ff:ff:ff` : cambia la dirección MAC de una interfaz de red
- `sudo ip l s ethN mtu 9000` : cambia el tamaño de MTU para una interfaz de red para usar marcos jumbo

## Links
- More information: https://manned.org/ip-link

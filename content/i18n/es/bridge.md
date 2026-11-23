# bridge

Muestra y manipula las direcciones y dispositivos de los puentes de red.

## Usage
- `bridge l` : lista todos los puentes y sus interfaces
- `bridge v` : muestra información de vlan de puerto
- `sudo bridge v a dev lanX vid vlan_id pvid tagged|untagged` : asigna una VLAN a un puerto
- `sudo bridge v d dev lanX vid vlan_id` : elimina una VLAN de un puerto
- `bridge mo` : vigila los cambios en las interfaces de bridge
- `bridge h` : muestra la ayuda

## Links
- More information: https://manned.org/bridge

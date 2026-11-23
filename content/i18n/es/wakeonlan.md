# wakeonlan

Envía paquetes a los PCs habilitados para wake-on-LAN (WOL).

## Usage
- `wakeonlan 01:02:03:04:05:06` : envía paquetes a todos los dispositivos de la red local (255.255.255.255) especificando una dirección MAC
- `wakeonlan 01:02:03:04:05:06 -i 192.168.178.2` : envía paquete a un dispositivo específico a través de una dirección IP
- `wakeonlan -n 01:02:03:04:05:06` : imprime los comandos, pero no los ejecutes (dry-run)
- `wakeonlan -q 01:02:03:04:05:06` : ejecuta en modo silencioso

## Links
- More information: https://manned.org/wakeonlan

# ip

Zeige und manipuliere routing, Geräte, Policy routing und Tunnel.

## Usage
- `ip a` : zeige Interfaces mit detaillierten Informationen
- `ip -br a` : zeige Interfaces mit kurzen Netzwerkinformationen
- `ip -br l` : zeige Interfaces mit kurzen link layer Informationen
- `ip r` : zeige die Routing Tabelle
- `ip n` : zeige Nachbarn (ARP Tabelle)
- `sudo ip l s interface up|down` : schalte ein bestimmtes Interface ein oder aus
- `sudo ip a add|delete ip_address/mask dev interface` : entferne oder füge eine IP zu einem Interface hinzu
- `sudo ip r a default via ip_address dev interface` : füge eine Standard Route hinzu

## Links
- More information: https://manned.org/ip.8

# ip address

IP Adressen Management Unterbefehl.

## Usage
- `ip a` : zeige Netzwerk-Interfaces mit ihren Adressen
- `ip a s up` : zeige nur die aktiven Netzwerk-Interfaces
- `ip a s eth0` : zeige Informationen über ein bestimmtes Interface
- `sudo ip a a ip_adresse dev eth0` : füge eine Adresse zu einem Interface hinzu
- `sudo ip a d ip_adresse dev eth0` : entferne eine Adresse von einem Interface
- `sudo ip a f eth0 scope global|host|link` : entfernt alle IP Adressen in einem speziellen Bereich von einem Interface

## Links
- More information: https://manned.org/ip-address

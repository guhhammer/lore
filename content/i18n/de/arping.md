# arping

Mit dem ARP Protokoll Hosts in einem Netzwerk entdecken und untersuchen. Nützlich für die Entdeckung von MAC-Adressen.

## Usage
- `sudo arping host_adresse` : pinge einen Host mit ARP Request Paketen
- `sudo arping -I interface host_adresse` : pinge einen Host auf einem spezifizierten Interface
- `sudo arping -f host_adresse` : pinge einen Host und höre nach der ersten Antwort auf
- `sudo arping -c anzahl host_adresse` : pinge einen Host für eine bestimmte Anzahl
- `sudo arping -U broadcast_adresse` : broadcaste ARP Request Pakete um die ARP Caches der Nachbarn zu aktualisieren
- `sudo arping -D -w 3 adresse_zum_checken` : sende ARP Requests mit einem 3 Sekunden Timeout um duplizierte IP-Adressen im Netzwerk zu erkennen

## Links
- More information: https://manned.org/arping

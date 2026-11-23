# arp-scan

Verstuur ARP-pakketten naar hosts (gespecificeerd als IP-adressen of hostnames) om het lokale netwerk te scannen.

## Usage
- `arp-scan -l` : scan het huidige lokale netwerk
- `arp-scan 10.0.0.1` : scan een specifieke host
- `arp-scan 192.168.1.1/24` : scan een IP-netwerk met een aangepast bitmasker
- `arp-scan 127.0.0.0-127.0.0.31` : scan een IP-netwerk binnen een aangepast bereik
- `arp-scan 10.0.0.0:255.255.255.0` : scan een IP-netwerk met een aangepast netmasker
- `arp-scan 10.0.0.1 -I interface_naam` : specificeer een interface om te gebruiken voor het scannen

## Links
- More information: https://github.com/royhills/arp-scan

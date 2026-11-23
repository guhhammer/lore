# ping.py

Controleer of een IPv4-host bereikbaar is door gebruik te maken van ICMP. Stuurt ICMP echo requests en luistert naar echo replies. Opmerking: Vereist root privileges voor raw socket toegang (bijvoorbeeld draaien met `sudo`). Onderdeel van de Impacket-suite.

## Usage
- `ping.py bron_ipv4 bestemmings_ipv4` : ping een host vanaf een opgegeven IPv4-bronadres
- `ping.py 192.168.1.10 192.168.1.100` : ping 192.168.1.100 vanaf 192.168.1.10

## Links
- More information: https://github.com/fortra/impacket

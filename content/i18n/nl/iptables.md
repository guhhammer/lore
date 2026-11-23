# iptables

Configureer tabellen, ketens en regels van de Linux kernel IPv4 firewall. Gebruik `ip6tables` om regels in te stellen voor IPv6 verkeer.

## Usage
- `sudo iptables -vnL --line-numbers` : bekijk ketens, regels, pakket/byte-tellers en regelnummers voor de filtertabel
- `sudo iptables -P keten regel` : stel een ketenbeleidsregel in (P)
- `sudo iptables -A keten -s ip_adres -j regel` : voeg regel toe aan ketenbeleid voor IP
- `sudo iptables -A keten -s ip_adres -p tcp|udp|icmp|... --dport poort -j regel` : voeg regel toe aan ketenbeleid voor IP met protocol en poort in overweging
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : voeg een NAT-regel toe om al het verkeer van het `192.168.0.0/24` subnet te vertalen naar de publieke IP van de host
- `sudo iptables -D keten regelnummer` : verwijder ketenregel

## Links
- More information: https://manned.org/iptables

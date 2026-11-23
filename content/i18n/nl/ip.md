# ip

Toon/manipuleer routing, apparaten, beleidsrouting en tunnels. Sommige subcommando's zoals `address` hebben hun eigen documentatie.

## Usage
- `ip a` : toon interfaces met gedetailleerde informatie
- `ip -br a` : toon interfaces met beknopte informatie over de netwerklaag
- `ip -br l` : toon interfaces met beknopte informatie over de linklaag
- `ip r` : toon de routingtabel
- `ip n` : toon buren (ARP-tabel)
- `sudo ip l s ethX up|down` : schakel een interface in/uit
- `sudo ip a add|delete ip_adres/mask dev ethX` : voeg een IP-adres toe aan een interface of verwijder het ervan
- `sudo ip r a default via ip_adres dev ethX` : voeg een standaardroute toe

## Links
- More information: https://manned.org/ip.8

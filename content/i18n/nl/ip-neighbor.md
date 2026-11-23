# ip neighbor

IP-subcommando voor het beheren van neighbor/ARP/NDP-tabellen.

## Usage
- `ip n` : toon de neighbor/ARP-tabelvermeldingen
- `sudo ip n f dev ethX` : verwijder vermeldingen in de neighbor-tabel op apparaat `ethX`
- `ip n g lookup_ip dev ethX` : voer een neighbor-lookup uit en geef een neighbor-vermelding terug
- `sudo ip n add|delete ip_adres lladdr mac_adres dev ethX nud reachable` : voeg een ARP-vermelding toe of verwijder deze voor het neighbor IP-adres op `ethX`
- `sudo ip n change|replace ip_adres lladdr nieuw_mac_adres dev ethX` : wijzig of vervang een ARP-vermelding voor het neighbor IP-adres op `ethX`
- `ip -6|-4 n` : toon alleen IPv6- of IPv4-neighbors

## Links
- More information: https://manned.org/ip-neighbour

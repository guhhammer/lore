# ip route list

IP-subcommando voor het weergeven van routeringstabellen.

## Usage
- `ip r l` : toon de `main` routeringstabel
- `ip r l t main|254` : toon de hoofdrouteringstabel (hetzelfde als het eerste voorbeeld)
- `ip r l t local|255` : toon de lokale routeringstabel
- `ip r l t all|unspec|0` : toon alle routeringstabellen
- `ip r l dev ethX` : toon alleen routes van een bepaald apparaat
- `ip r l s link` : toon een lijst van routes binnen een bepaald bereik
- `ip r l c` : toon de routeringscache
- `ip -6|-4 r` : toon alleen IPv6- of IPv4-routes

## Links
- More information: https://manned.org/ip-route

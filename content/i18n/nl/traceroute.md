# traceroute

Toon het pad dat pakketjes volgen naar een netwerkhost.

## Usage
- `traceroute example.com` : traceroute naar een host
- `traceroute -n example.com` : schakel IP-adres en hostnaam mapping uit
- `traceroute -w 0.5 example.com` : specificeer wachttijd in seconden voor antwoord
- `traceroute -q 5 example.com` : specificeer het aantal queries per hop
- `traceroute example.com 42` : specificeer de grootte in bytes van het peilpakket
- `traceroute --mtu example.com` : bepaal de MTU naar de bestemming
- `traceroute -I example.com` : gebruik ICMP in plaats van UDP voor tracerouting

## Links
- More information: https://manned.org/traceroute

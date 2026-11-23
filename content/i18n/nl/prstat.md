# prstat

Rapporteer statistieken van actieve processen.

## Usage
- `prstat` : bekijk alle processen en rapporteer de statistieken gesorteerd op basis van CPU-gebruik
- `prstat -s rss` : bekijk alle processen en rapporteer de statistieken gesorteerd op basis van geheugengebruik
- `prstat -t` : bekijk het totaalgebruik voor elke gebruiker
- `prstat -m` : bekijk de microstate process accounting informatie
- `prstat -c -n 5 -s cpu 1` : toon de 5 meest CPU intensieve processen elke seconde

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat

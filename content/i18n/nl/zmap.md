# zmap

Snelle, opensource netwerkscanner voor internetbrede onderzoeken.

## Usage
- `sudo zmap -p poort subnet` : scan een subnet of de volledige IPv4-ruimte op een specifieke TCP-poort (standaard: 80)
- `sudo zmap -p poort1,poort2-poort3,... subnet` : scan specifieke poorten of poortbereiken over een subnet
- `sudo zmap -o pad/naar/uitvoerbestand.csv -f "saddr,daddr,sport,dport" subnet` : schrijf resultaten weg naar een CSV-bestand met aangepaste velden
- `sudo zmap -r pakketten_per_seconde subnet` : beperk de scansnelheid tot een specifiek aantal pakketten per seconde
- `zmap -d subnet` : simuleer het uitvoeren van zmap zonder pakketten te versturen
- `sudo zmap -b pad/naar/blocklist.txt subnet` : sluit subnetten uit met behulp van een blocklistbestand in CIDR-notatie
- `sudo zmap -S bron_ip subnet` : stel een specifiek bron-IP in voor scanpakketten
- `sudo zmap -p poort1,poort2-poort3 -n 1000 subnet` : beperk het aantal/percentage doelen om te onderzoeken (bijv. 1000 IP/poort-paren)

## Links
- More information: https://manned.org/zmap

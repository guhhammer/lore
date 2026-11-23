# certutil

Beheer sleutels en certificaten in zowel NSS-databases als andere NSS-tokens.

## Usage
- `certutil -N -d .` : maak een Nieuwe certificaatdatabase aan in de huidige directory
- `certutil -L -d .` : toon een Lijst van alle certificaten in een database
- `certutil -K -d . -f pad/naar/wachtwoordbestand.txt` : toon alle privésleutels (K) in een database door het wachtwoordbestand (f) op te geven
- `certutil -A -n "server_certificaat" -t ",," -i pad/naar/bestand.crt -d .` : voeg (A) het ondertekende certificaat toe aan de database van de aanvrager, met een bijnaam, vertrouwensattributen en een invoer-CRT-bestand
- `certutil -S -f pad/naar/wachtwoordbestand.txt -d . -t ",," -c "server_certificaat" -n "server_naam" -g 2048 -s "CN=common_name,O=organisatie"` : voeg subject alternative names toe aan een certificaat met een specifieke sleutelgrootte (g)

## Links
- More information: https://manned.org/certutil

# wget

Download bestanden vanaf het internet. Ondersteunt HTTP, HTTPS en FTP.

## Usage
- `wget https://example.com/foo` : download de inhoud van een URL naar een bestand (in dit geval genaamd "foo")
- `wget -O pad/naar/bestand https://example.com/foo` : download de inhoud van een URL naar een specifiek bestand
- `wget -pkw 3 https://example.com/een_pagina.html` : download één webpagina en alle bijbehorende bronnen met een interval van 3 seconden tussen verzoeken (scripts, stylesheets, afbeeldingen, etc.)
- `wget -mnp https://example.com/een_pad/` : download alle vermelde bestanden binnen een map en zijn submappen (downloadt geen embedded pagina-elementen)
- `wget --limit-rate 300k -t 100 https://example.com/een_pad/` : beperk de downloadsnelheid en het aantal verbindingspogingen
- `wget --user gebruikersnaam --password wachtwoord https://example.com` : download een bestand van een HTTP-server met behulp van Basic Auth (werkt ook met FTP)
- `wget -c https://example.com` : hervat een onvolledige download
- `wget -P pad/naar/map -i pad/naar/URLs.txt` : download alle URL's die zijn opgeslagen in een tekstbestand naar een specifieke map

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html

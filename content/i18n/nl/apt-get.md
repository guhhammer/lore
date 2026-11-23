# apt-get

Hulpprogramma voor pakketbeheer van Debian en Ubuntu. Zoek naar pakketten met `apt-cache`. Het is aanbevolen om `apt` te gebruiken wanneer het interactief wordt gebruikt in Ubuntu versies 16.04 en later.

## Usage
- `sudo apt-get update` : werk de lijst van beschikbare pakketten en versies bij (het wordt aanbevolen dit uit te voeren voor elk ander `apt-get` commando)
- `sudo apt-get install pakket` : installeer een pakket, of werk het bij naar de nieuwste beschikbare versie
- `sudo apt-get remove pakket` : verwijder een pakket
- `sudo apt-get purge pakket` : verwijder een pakket en zijn configuratiebestanden
- `sudo apt-get upgrade` : upgrade alle geïnstalleerde pakketten naar hun nieuwste beschikbare versies
- `sudo apt-get autoclean` : schoon de lokale repository op - verwijder pakketbestanden (`.deb`) van onderbroken downloads die niet langer kunnen worden gedownload
- `sudo apt-get autoremove` : verwijder alle pakketten die niet meer nodig zijn
- `sudo apt-get dist-upgrade` : upgrade geïnstalleerde pakketten (zoals `upgrade`), maar verwijder verouderde pakketten en installeer aanvullende pakketten om aan nieuwe dependencies te voldoen

## Links
- More information: https://manned.org/apt-get.8

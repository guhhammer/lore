# rename.util

Hernoem meerdere bestanden. WAARSCHUWING: dit commando overschrijft bestanden zonder te vragen, tenzij de `--no-act` optie wordt gebruikt. Opmerking: deze pagina verwijst naar het commando uit het `util-linux` pakket.

## Usage
- `rename string vervanging *` : hernoem bestanden met eenvoudige vervangingen (vervang een string door een vervanging waar het ook gevonden wordt)
- `rename -vn string vervanging *` : simuleer het uitvoeren van het programma zonder iets te doen
- `rename -o string vervanging *` : overschrijf geen bestaande bestanden
- `rename .ext .bak *.ext` : wijzig bestandsextensies
- `rename '' 'voorvoegsel' *` : voeg een voorvoegsel toe aan het begin van alle bestandsnamen in de huidige map
- `rename voorvoegsel voorvoegsel00 voorvoegsel? && rename voorvoegsel voorvoegsel0 voorvoegsel??` : hernoem een groep opeenvolgend genummerde bestanden met nul-opvulling van de nummers tot 3 cijfers

## Links
- More information: https://manned.org/rename

# make

Taakuitvoerder voor doelen beschreven in Makefile. Wordt meestal gebruikt om de compilatie van een uitvoerbaar bestand uit broncode te beheren.

## Usage
- `make` : roep het eerste doel aan dat in de Makefile is gespecificeerd (meestal "all" genoemd)
- `make install|check|clean|uninstall|...` : roep een specifiek doel aan
- `make -j 4 doel` : roep een specifiek doel aan en voer 4 taken tegelijk uit in parallel
- `make -f pad/naar/bestand` : gebruik een specifieke Makefile
- `make -C pad/naar/map` : voer make uit vanuit een andere map
- `make -B doel` : forceer het maken van een doel, zelfs als bronbestanden ongewijzigd zijn
- `make doel variabele=nieuwe_waarde` : overschrijf een variabele die in de Makefile is gedefinieerd
- `make -e doel` : overschrijf variabelen die in de Makefile zijn gedefinieerd door de omgeving

## Links
- More information: https://www.gnu.org/software/make/manual/make.html

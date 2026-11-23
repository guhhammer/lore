# fossil

Gedistribueerd versiebeheersysteem met een ingebouwde wiki, bugtracker en webinterface. Sommige subcommando's zoals `commit` hebben hun eigen documentatie.

## Usage
- `fossil init repository_naam.fossil` : maak een nieuwe lege Fossil-repository aan
- `fossil clone externe_url` : maak een lokale kopie van een externe repository
- `fossil status` : toon een overzicht van de huidige status van de repository
- `fossil add pad/naar/bestand` : voeg een nieuw bestand toe
- `fossil rm pad/naar/bestand` : verwijder een bestand
- `fossil ci -m "opmerking"` : commit alle toegevoegde wijzigingen
- `fossil push externe_url` : stuur wijzigingen van de lokale repository naar een externe repository
- `fossil pull externe_url` : haal de wijzigingen van een externe repository op naar een lokale repository

## Links
- More information: https://fossil-scm.org/home/help

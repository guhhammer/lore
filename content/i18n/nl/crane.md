# crane

Hulpmiddel voor het beheren van containerimages. Sommige subcommando's zoals `pull`, `push`, `copy`, etc. hebben hun eigen documentatie.

## Usage
- `crane auth login registry -u gebruiker -p wachtwoord` : log in op een registry
- `crane catalog registry --full-ref` : toon de repositories in een registry
- `crane ls repository -o` : toon de tags in een repository
- `crane pull image tarball` : haal externe images op door middel van referentie en sla de inhoud ervan lokaal op
- `crane push pad/naar/map_of_tarball image` : push lokale inhoud van images naar een externe registry
- `crane tag image tag` : tag efficiënt een externe image
- `crane cp bron doel -a` : kopieer efficiënt een externe image, waarbij de digest-waarde behouden blijft
- `crane delete image` : verwijder een image-referentie van zijn registry

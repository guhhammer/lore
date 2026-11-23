# crane registry

Dit commando biedt een registry-implementatie op een automatisch gekozen poort (`:0`), `$PORT` of `--address`. Het commando blokkeert terwijl de server pushes en pulls accepteert en de inhoud kan worden opgeslagen in het geheugen en op de schijf.

## Usage
- `crane registry serve` : start een registry-implementatie
- `crane registry serve --address adres_naam` : specificeer het adres om naar te luisteren
- `crane registry serve --disk pad/naar/map` : sla blobs op in een specifieke map
- `crane registry -h` : toon de help voor `crane registry`
- `crane registry serve -h` : toon de help voor `crane registry serve`

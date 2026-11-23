# clamscan

Een command-line virusscanner.

## Usage
- `clamscan pad/naar/bestand` : scan een bestand op kwetsbaarheden
- `clamscan -r pad/naar/map` : scan alle bestanden recursief in een specifieke map
- `commando | clamscan -` : scan data van `stdin`
- `clamscan -d pad/naar/database_bestand_of_map` : specificeer een virusdatabasebestand of map van bestanden
- `clamscan -i` : scan de huidige map en toon alleen geïnfecteerde bestanden
- `clamscan -l pad/naar/log_bestand` : sla het scanrapport op in een logbestand
- `clamscan --move pad/naar/quarantaine_map` : verplaats geïnfecteerde bestanden naar een specifieke map
- `clamscan --remove yes` : verwijder geïnfecteerde bestanden

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan

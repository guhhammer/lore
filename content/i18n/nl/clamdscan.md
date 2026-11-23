# clamdscan

Een command-line virusscanner die gebruikmaakt van de ClamAV Daemon.

## Usage
- `clamdscan pad/naar/bestand_of_map` : scan een bestand of map op kwetsbaarheden
- `commando | clamdscan -` : scan data van `stdin`
- `clamdscan --infected` : scan de huidige map en toon alleen geïnfecteerde bestanden
- `clamdscan --log pad/naar/log_bestand` : sla het scanrapport op in een logbestand
- `clamdscan --move pad/naar/quarantaine_map` : verplaats geïnfecteerde bestanden naar een specifieke map
- `clamdscan --remove` : verwijder geïnfecteerde bestanden
- `clamdscan --multiscan` : gebruik meerdere threads voor het scannen van een map
- `clamdscan --fdpass` : geef de bestandsdescriptor door in plaats van het bestand naar de daemon

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan

# rcp

Kopieer bestanden tussen lokale en externe systemen. Het imiteert het gedrag van het `cp`-commando, maar werkt tussen verschillende machines.

## Usage
- `rcp pad/naar/lokaal_bestand gebruikersnaam@remote_host:/pad/naar/bestemming/` : kopieer een bestand naar een externe host
- `rcp -r pad/naar/lokale_map gebruikersnaam@remote_host:/pad/naar/bestemming/` : kopieer een map recursief
- `rcp -p pad/naar/lokaal_bestand gebruikersnaam@remote_host:/pad/naar/bestemming/` : behoud de bestandseigenschappen
- `rcp -f pad/naar/lokaal_bestand gebruikersnaam@remote_host:/pad/naar/bestemming/` : forceer kopiëren zonder bevestiging

## Links
- More information: https://www.gnu.org/software/inetutils/manual/inetutils.html#rcp-invocation

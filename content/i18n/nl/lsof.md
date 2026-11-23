# lsof

Toon open bestanden en de bijbehorende processen. Opmerking: rootrechten zijn vereist om bestanden te tonen die door anderen zijn geopend.

## Usage
- `lsof pad/naar/bestand` : zoek de processen die een gegeven bestand open hebben staan
- `lsof -i :poort` : zoek het proces dat een lokale internetpoort heeft geopend
- `lsof -t pad/naar/bestand` : toon alleen het proces-ID (PID)
- `lsof -u gebruikersnaam` : toon bestanden geopend door de gegeven gebruiker
- `lsof -c proces_of_commando_naam` : toon bestanden geopend door het gegeven commando of proces
- `lsof -p proces_id` : toon bestanden geopend door een specifiek proces, gegeven zijn PID
- `lsof +D pad/naar/map` : toon open bestanden in een map
- `lsof -i6TCP:poort -sTCP:LISTEN -n -P` : zoek het proces dat luistert op een lokale IPv6 TCP-poort en converteer geen netwerk- of poortnummers

## Links
- More information: https://manned.org/lsof

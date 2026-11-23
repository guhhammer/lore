# nc

Netcat is een veelzijdig hulpprogramma voor het omleiden van IO naar een netwerkstream.

## Usage
- `nc <bestandsnaam -l -p poort` : start een luisteraar op de opgegeven TCP-poort en stuur er een bestand in
- `nc host poort >ontvangen_bestandsnaam` : maak verbinding met een doelluisteraar op de opgegeven poort en ontvang er een bestand uit
- `nc -v -z -w timeout_in_seconden host start_poort-eind_poort` : scan de open TCP-poorten van een opgegeven host
- `nc -l -p poort -e shell_executable` : start een luisteraar op de opgegeven TCP-poort en geef uw lokale shell toegang tot de verbonden partij (dit is gevaarlijk en kan worden misbruikt)
- `nc host poort -e shell_executable` : maak verbinding met een doelluisteraar en geef uw lokale shell toegang tot de externe partij (dit is gevaarlijk en kan worden misbruikt)
- `nc -l -p lokale_poort | nc host externe_poort` : fungeer als een proxy en stuur gegevens door van een lokale TCP-poort naar de opgegeven externe host
- `echo -e "GET / HTTP/1.1\nHost: host\n\n" | nc host 80` : stuur een HTTP GET verzoek

## Links
- More information: https://manned.org/nc

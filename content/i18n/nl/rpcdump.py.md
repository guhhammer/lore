# rpcdump.py

Informatie over RPC-eindpunten op afstand dumpen via de Endpoint Mapper. Onderdeel van de Impacket-suite.

## Usage
- `rpcdump.py domein/gebruikersnaam:wachtwoord@doel` : dump RPC eindpunten met gebruikersnaam en wachtwoord
- `rpcdump.py -hashes LMHASH:NTHASH domein/gebruikersnaam:wachtwoord@doel` : dump RPC eindpunten met NTLM hashes
- `rpcdump.py -target-ip doel_ip domein/gebruikersnaam:wachtwoord@doel` : geef expliciet een doel-IP-adres op (handig als de doelnaam een NetBIOS-naam is)
- `rpcdump.py -port poort_nummer domein/gebruikersnaam:wachtwoord@doel` : maak verbinding met een specifieke poort (standaard is 135 voor RPC Endpoint Mapper)
- `rpcdump.py -debug domein/gebruikersnaam:wachtwoord@doel` : schakel debug-uitvoer in

## Links
- More information: https://github.com/fortra/impacket

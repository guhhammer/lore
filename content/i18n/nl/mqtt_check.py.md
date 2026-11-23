# mqtt_check.py

Eenvoudig hulpprogramma voor het testen en valideren van MQTT-aanmeldingsgegevens. Onderdeel van de Impacket-suite.

## Usage
- `mqtt_check.py domein/gebruikersnaam:wachtwoord@doel_naam` : controleer MQTT-aanmeldingsgegevens voor een doel (hostnaam van de MQTT-broker)
- `mqtt_check.py -client-id client_id domein/gebruikersnaam:wachtwoord@doel_naam` : geef een aangepaste client-ID op voor authenticatie
- `mqtt_check.py -ssl domein/gebruikersnaam:wachtwoord@doel_naam` : schakel SSL in voor de verbinding
- `mqtt_check.py -port poort domein/gebruikersnaam:wachtwoord@doel_naam` : maak verbinding met een specifieke poort (standaard is 1883)
- `mqtt_check.py -debug domein/gebruikersnaam:wachtwoord@doel_naam` : schakel debug-uitvoer in
- `mqtt_check.py --help` : toon de help

## Links
- More information: https://github.com/fortra/impacket

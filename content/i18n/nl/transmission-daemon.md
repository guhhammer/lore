# transmission-daemon

Daemon bediend met `transmission-remote` of de webinterface.

## Usage
- `transmission-daemon` : start een headless `transmission` sessie
- `transmission-daemon -c pad/naar/map` : start en bewaak een specifieke map voor nieuwe torrents
- `transmission-daemon -d >pad/naar/bestand.json` : dump daemon-instellingen in JSON formaat
- `transmission-daemon -t -u gebruikersnaam -v wachtwoord -p 9091 -a 127.0.0.1` : start met specifieke instellingen voor de webinterface

## Links
- More information: https://manned.org/transmission-daemon

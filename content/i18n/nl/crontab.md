# crontab

Plan cron jobs zodat deze volgens een tijdsinterval voor de huidige gebruiker worden uitgevoerd.

## Usage
- `crontab -e` : pas het crontab-bestand aan voor de huidige gebruiker
- `sudo crontab -e -u gebruiker` : pas het crontab-bestand aan voor een specifieke gebruiker
- `crontab pad/naar/bestand` : vervang de huidige crontab met de inhoud van een opgegeven bestand
- `crontab -l` : bekijk een lijst van bestaande cron jobs voor de huidige gebruiker
- `crontab -r` : verwijder alle cron jobs voor de huidige gebruiker
- `0 10 * * * commando_om_uit_te_voeren` : voorbeeld cron job die iedere dag om 10:00 draait (* betekent elke waarde)
- `*/10 * * * * commando_om_uit_te_voeren` : voorbeeld cron job die iedere 10 minuten een commando uitvoert
- `30 2 * * Fri /pad/naar/script.sh` : voorbeeld cron job die iedere vrijdag om 02:30 een specifiek script draait

## Links
- More information: https://manned.org/crontab

# at

Voer commando's één keer uit op een later tijdstip. Resultaten worden naar de e-mail van de gebruiker gestuurd.

## Usage
- `at now + 5 minutes` : maak commando's interactief en voer ze over 5 minuten uit (druk op `<Ctrl d>` wanneer klaar)
- `at hh:mm` : maak commando's interactief en voer ze uit op een specifiek tijdstip
- `echo "commando" | at 1000` : voer een commando uit vanuit `stdin` om 10:00 uur vandaag
- `at -f pad/naar/bestand 9:30 PM Tue` : voer commando's uit vanuit een opgegeven bestand volgende dinsdag
- `at -l` : toon alle jobs in de wachtrij voor de huidige gebruiker (hetzelfde als `atq`)
- `at -c job_nummer` : toon een specifieke job

## Links
- More information: https://manned.org/at

# renice

Verander de scheduleringsprioriteit/niceness van lopende processen. Niceness waarden variëren van -20 (meest gunstig voor het proces) tot 19 (minst gunstig voor het proces).

## Usage
- `renice -n 3 -p proces_id` : verhoog/verlaag de prioriteit van een lopend proces
- `renice -n -4 -u gebruikers_id|gebruikersnaam` : verhoog/verlaag de prioriteit van alle processen die eigendom zijn van een gebruiker (user)
- `renice -n 5 -g proces_groep` : verhoog/verlaag de prioriteit van alle processen die behoren tot een procesgroep

## Links
- More information: https://manned.org/renice.1p

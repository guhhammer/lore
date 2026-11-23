# passwd

Verander het wachtwoord van een gebruiker.

## Usage
- `passwd` : verander het wachtwoord van de huidige gebruiker interactief
- `sudo passwd gebruikersnaam` : verander het wachtwoord van een specifieke gebruiker
- `passwd -S` : toon de huidige status van de gebruiker
- `passwd -d` : maak het wachtwoord van het account leeg (hiermee wordt de opgegeven account wachtwoordloos)
- `yes wachtwoord | passwd` : stel het wachtwoord programmatisch in (handig voor installatiescripts)

## Links
- More information: https://manned.org/passwd

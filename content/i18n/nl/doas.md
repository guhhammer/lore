# doas

Voer een commando uit als een andere gebruiker.

## Usage
- `doas commando` : voer een commando uit als root
- `doas -u gebruiker commando` : voer een commando uit als een andere gebruiker
- `doas -s` : start de standaard shell als root
- `doas -C pad/naar/configuratiebestand commando` : parse een configuratiebestand en controleer of de uitvoering van het commando als een andere gebruiker toegestaan is
- `doas -L` : zorg ervoor dat `doas` om een wachtwoord vraagt, zelfs als het eerder is opgegeven

## Links
- More information: https://man.openbsd.org/doas

# addcomputer.py

Voeg een computeraccount toe aan het domein. Onderdeel van de Impacket-suite.

## Usage
- `addcomputer.py -computer-name COMPUTER_NAAM$ -computer-pass computer_wachtwoord domein/gebruikersnaam:wachtwoord` : voeg een computer toe met een specifieke naam en wachtwoord
- `addcomputer.py -no-add -computer-name COMPUTER_NAAM$ -computer-pass computer_wachtwoord domein/gebruikersnaam:wachtwoord` : stel alleen een nieuw wachtwoord in op een bestaande computer
- `addcomputer.py -delete -computer-name COMPUTER_NAAM$ domein/gebruikersnaam:wachtwoord` : verwijder een bestaand computeraccount
- `addcomputer.py -k -no-pass domein/gebruikersnaam@hostnaam` : voeg een computer toe met behulp van Kerberos-authenticatie
- `addcomputer.py -method LDAPS -port 636 domein/gebruikersnaam:wachtwoord` : voeg een computer toe via LDAPS (poort 636) in plaats van SAMR (poort 445)
- `addcomputer.py -dc-host hostnaam domein/gebruikersnaam:wachtwoord` : geef de exacte domeincontroller op als er meerdere DC's bestaan

## Links
- More information: https://github.com/fortra/impacket

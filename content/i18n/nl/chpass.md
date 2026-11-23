# chpass

Gebruikersdatabase-informatie toevoegen of wijzigen, inclusief login shell en wachtwoord. Opmerking: het is niet mogelijk om een gebruikerswachtwoord te wijzigen op een Open Directory systeem, gebruik hiervoor `passwd`.

## Usage
- `su -c chpass` : voeg toe of pas interactief de gebruikersdatabase-informatie aan voor de huidige gebruiker
- `chpass -s pad/naar/shell` : stel een specifieke login shell in voor de huidige gebruiker
- `chpass -s pad/naar/shell gebruikersnaam` : stel een login shell in voor een specifieke gebruiker
- `chpass -l locatie -s pad/naar/shell gebruikersnaam` : pas het gebruikersrecord aan in de directory node op de opgegeven locatie
- `chpass -u authnaam -s pad/naar/shell gebruikersnaam` : gebruik de opgegeven gebruikersnaam bij het authenticeren van de directory node die de gebruiker bevat

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass

# su

Wissel shell naar een andere gebruiker.

## Usage
- `su` : wissel naar superuser (vereist het rootwachtwoord)
- `su gebruikersnaam` : wissel naar een gegeven gebruiker (vereist het wachtwoord van de gebruiker)
- `su - gebruikersnaam` : wissel naar een gegeven gebruiker en simuleer een volledige login shell
- `su - gebruikersnaam -c "commando"` : voer een commando uit als een andere gebruiker
- `su -s /pad/naar/shell gebruikersnaam` : wissel naar een bepaalde gebruiker en gebruik een specifieke shell (bijv. Zsh, fish, Bash)
- `su -h` : toon de help
- `su -V` : toon de versie

## Links
- More information: https://manned.org/su

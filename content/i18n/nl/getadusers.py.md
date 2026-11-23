# getadusers.py

Haal een lijst met gebruikers op van Active Directory, inclusief attributen zoals laatste logon timestamp en email. Onderdeel van de Impacket-suite.

## Usage
- `GetADUsers.py -all -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord` : som alle Active Directory gebruikers en hun attributen op
- `GetADUsers.py -user gebruiker -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord` : verkrijg informatie voor een specifieke gebruiker
- `GetADUsers.py -all -dc-ip domain_controller_ip -hashes LM_Hash:NT_Hash domein/gebruikersnaam` : extraheer gebruikersdetails door gebruik te maken van pass-the-hash authentication
- `GetADUsers.py -all -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord >pad/naar/output.txt` : sla de output op in een bestand

## Links
- More information: https://github.com/fortra/impacket

# getuserspns.py

Haal Service Principal Names (SPN's) op die gekoppeld zijn aan Active Directory gebruikersaccounts. Onderdeel van de Impacket-suite.

## Usage
- `GetUserSPNs.py domein/gebruikersnaam:wachtwoord -dc-ip domain_controller_ip` : som gebruikersaccounts met een SPN op en vraag hun Kerberos TGS tickets op
- `GetUserSPNs.py domein/gebruikersnaam -hashes LM_Hash:NT_Hash -dc-ip domain_controller_ip` : gebruik pass-the-hash authenticatie
- `GetUserSPNs.py domein/gebruikersnaam:wachtwoord -dc-ip domain_controller_ip -outputfile pad/naar/uitvoerbestand` : sla de uitvoer op in een bestand
- `GetUserSPNs.py domein/gebruikersnaam:wachtwoord -dc-ip domain_controller_ip -request` : vraag alleen TGS tickets op
- `GetUserSPNs.py domein/gebruikersnaam -dc-ip domain_controller_ip -hashes LM_Hash:NT_Hash -request` : vraag alleen TGS tickets aan met pass-the-hash authenticatie

## Links
- More information: https://github.com/fortra/impacket

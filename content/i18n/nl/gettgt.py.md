# gettgt.py

Verzoek een Ticket Granting Ticket (TGT). Onderdeel van de Impacket-suite.

## Usage
- `getTGT.py domein/gebruikersnaam:wachtwoord` : verzoek een TGT met behulp van een wachtwoord
- `getTGT.py -hashes LM_Hash:NT_Hash domein/gebruikersnaam` : verzoek een TGT met behulp van NTLM hashes
- `getTGT.py -k -no-pass domein/gebruikersnaam` : gebruik Kerberos authenticatie (van bestaande ccache, geen wachtwoord nodig)
- `getTGT.py -aesKey aes_key domein/gebruikersnaam` : verzoek een TGT met behulp van een AES key (128 of 256 bits)
- `getTGT.py -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord` : specificeer een domain controller IP
- `getTGT.py -service SPN domein/gebruikersnaam:wachtwoord` : vraag direct een serviceticket aan (AS-REQ) voor een specifieke SPN

## Links
- More information: https://github.com/fortra/impacket

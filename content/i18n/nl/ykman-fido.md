# ykman fido

Beheer YubiKey FIDO-applicaties.

## Usage
- `ykman fido info` : toon algemene informatie over de FIDO2-applicatie
- `ykman fido access change-pin` : verander de FIDO pin
- `ykman fido credentials list` : toon een lijst van residente inloggegevens die opgeslagen zijn op de YubiKey
- `ykman fido credentials delete inloggegevens_id` : verwijder specifieke residente inloggegevens van de YubiKey
- `ykman fido fingerprints list` : toon vingerafdrukken opgeslagen op de YubiKey (vereist een sleutel met een vingerafdruksensor)
- `ykman fido fingerprints add naam` : voeg een nieuwe vingerafdruk toe aan de YubiKey
- `ykman fido fingerprints delete naam` : verwijder een vingerafdruk van de YubiKey
- `ykman fido reset` : wis alle FIDO inloggegevens (je moet dit doen nadat je het aantal pogingen voor de pin hebt overschreden)

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/FIDO_Commands.html

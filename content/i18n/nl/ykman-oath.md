# ykman oath

Beheer de OATH YubiKey-applicatie. Een `keyword` kan onderdeel zijn van de naam of van de indiener.

## Usage
- `ykman oath info` : toon algemene informatie over de OATH-applicatie
- `ykman oath access change` : verander het wachtwoord dat de OATH accounts beschermt (voeg `--clear` toe om het te verwijderen)
- `ykman oath accounts add -i indiener naam` : voeg een nieuw account toe (de indiener is optioneel)
- `ykman oath accounts list` : toon alle accounts (met hun indieners)
- `ykman oath accounts code keyword` : toon alle accounts met hun huidige TOTP/HOTP codes (optioneel kan deze lijst gefilterd worden met een keyword)
- `ykman oath accounts rename keyword indiener:naam|naam` : hernoem een account
- `ykman oath accounts delete keyword` : verwijder een account
- `ykman oath reset` : verwijder alle accounts en herstel de fabrieksinstellingen

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/OATH_Commands.html

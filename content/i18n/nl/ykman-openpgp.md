# ykman openpgp

Beheer de OpenPGP YubiKey-applicatie. Opmerking: je dient `gpg --card-edit` te gebruiken voor sommige instellingen.

## Usage
- `ykman openpgp info` : toon algemene informatie over de OpenPGP-applicatie
- `ykman openpgp access set-retries 3 3 3` : stel het aantal herstelpogingen in voor respectievelijk de gebruikers pin, herstelcode en admin pin
- `ykman openpgp access change-pin|reset-code|admin-pin` : verander de gebruikers pin, herstelcode of admin pin
- `ykman openpgp reset` : herstel de OpenPGP-applicatie naar fabrieksinstellingen (je moet dit doen nadat je het aantal pogingen voor de admin pin hebt overschreden)

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/OpenPGP_Commands.html

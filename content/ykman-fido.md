# ykman fido

Manage YubiKey FIDO applications.

## Usage
- `ykman fido info` : display general information about the FIDO2 application
- `ykman fido access change-pin` : change the FIDO pin
- `ykman fido credentials list` : list resident credentials stored on the YubiKey
- `ykman fido credentials delete credential_id` : delete a resident credential from the YubiKey
- `ykman fido fingerprints list` : list fingerprints stored on the YubiKey (requires a key with a fingerprint sensor)
- `ykman fido fingerprints add name` : add a new fingerprint to the YubiKey
- `ykman fido fingerprints delete name` : delete a fingerprint from the YubiKey
- `ykman fido reset` : wipe all FIDO credentials (you have to do this after exceeding the number of PIN retry attempts)

## Links
- More information: https://docs.yubico.com/software/yubikey/tools/ykman/FIDO_Commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/ykman-fido

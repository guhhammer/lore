# fido2-token

Find and manage a FIDO2 authenticator.

## Usage
- `fido2-token -L` : list all FIDO2 authenticators
- `fido2-token -I device` : display information about a specific authenticator
- `fido2-token -L -r device` : list relying parties with discoverable credentials stored on an authenticator
- `fido2-token -L -k relying_party_id device` : list discoverable credentials for a specific relying party (usually a domain name)
- `fido2-token -D -i credential_id device` : delete a discoverable credential by its base64-encoded ID
- `fido2-token -S|-C device` : set or change the authenticator's PIN
- `fido2-token -R device` : reset an authenticator to default factory settings

## Categories
hardware

## Links
- More information: https://developers.yubico.com/libfido2/Manuals/fido2-token.html
- tldr page: https://tldr.inbrowser.app/pages/common/fido2-token

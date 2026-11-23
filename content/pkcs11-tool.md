# pkcs11-tool

Utility for managing and using PKCS #11 security tokens.

## Usage
- `pkcs11-tool --module path/to/module.so -L -T` : list slots and their potential token, using a specific module (e.g. `/usr/lib/softhsm/libsofthsm2.so`)
- `pkcs11-tool -O -p auth_pin --slot slot_id` : list objects in a specific slot. (Note: `slot_id` is not the slot index shown as "Slot X")
- `pkcs11-tool --slot slot_id -p auth_pin -y cert|privkey|pubkey|secrkey|data|... -a "label" -d 01 -w path/to/cert.crt` : create a new object with a specific label and type
- `pkcs11-tool --slot slot_id -p auth_pin -y cert|privkey|pubkey|secrkey|data|... -a "label" -b` : delete an object by its label and type

## Categories
development, system

## Links
- More information: https://github.com/OpenSC/OpenSC/wiki/Using-pkcs11-tool-and-OpenSSL
- tldr page: https://tldr.inbrowser.app/pages/common/pkcs11-tool

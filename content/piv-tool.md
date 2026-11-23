# piv-tool

Smart card utility for HSPD-12 PIV cards.

## Usage
- `piv-tool --serial` : print the card serial number derived from the CHUID object, if any
- `piv-tool --name` : print the name of the inserted card (driver)
- `piv-tool --admin argument` : authenticate to the card using a 2DES, 3DES or AES key
- `piv-tool --genkey argument` : generate a key pair on the card and output the public key

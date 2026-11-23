# cryptsetup-luksformat

Inizializza una partizione LUKS e lo slot di chiave iniziale (0) con una passphrase o un keyfile. Nota: Questa operazione sovrascrive tutti i dati sulla partizione.

## Usage
- `cryptsetup luksFormat /dev/sdXY` : inizializza un volume LUKS con una passphrase
- `cryptsetup luksFormat /dev/sdXY percorso/al/keyfile` : inizializza un volume LUKS con un keyfile
- `cryptsetup luksFormat --label label /dev/sdXY` : inizializza un volume LUKS con una passphrase e imposta la sua etichetta

## Links
- More information: https://manned.org/cryptsetup-luksFormat

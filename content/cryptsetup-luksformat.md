# cryptsetup-luksformat

Initialize a LUKS partition and the initial key slot (0) with a passphrase or keyfile. Note: This operation overwrites all data on the partition.

## Usage
- `cryptsetup luksFormat /dev/sdXY` : initialize a LUKS volume with a passphrase
- `cryptsetup luksFormat /dev/sdXY path/to/keyfile` : initialize a LUKS volume with a keyfile
- `cryptsetup luksFormat --label label /dev/sdXY` : initialize a LUKS volume with a passphrase and set its label

## Categories
disks

## Links
- More information: https://manned.org/cryptsetup-luksFormat
- tldr page: https://tldr.inbrowser.app/pages/linux/cryptsetup-luksformat

# cryptsetup

Manage plain `dm-crypt` and LUKS (Linux Unified Key Setup) encrypted volumes. Some subcommands such as `luksFormat` have their own usage documentation.

## Usage
- `cryptsetup luksFormat /dev/sdXY` : initialize a LUKS volume with a passphrase (overwrites all data on the partition)
- `cryptsetup open /dev/sdXY mapping_name` : open a LUKS volume and create a decrypted mapping at `/dev/mapper/mapping_name`
- `cryptsetup status mapping_name` : display information about a mapping
- `cryptsetup close mapping_name` : remove an existing mapping
- `cryptsetup luksChangeKey /dev/sdXY` : change a LUKS volume's passphrase
- `cryptsetup luksDump /dev/sdXY` : display LUKS header information and key slot metadata of an encrypted device

## Categories
danger, disks, documents, monitoring

## Links
- More information: https://manned.org/cryptsetup
- tldr page: https://tldr.inbrowser.app/pages/linux/cryptsetup

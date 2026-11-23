# cryptsetup open

Create a decrypted mapping of an encrypted volume. Note: Enabling TRIM may leak minimal information (freed blocks, possibly revealing the filesystem in use), but is still recommended since the data stays safe and SSDs without TRIM wear out faster.

## Usage
- `cryptsetup open /dev/sdXY mapping_name` : open a LUKS volume and create a decrypted mapping at `/dev/mapper/mapping_name`
- `cryptsetup open -k path/to/file /dev/sdXY mapping_name` : use a keyfile instead of a passphrase
- `cryptsetup open --allow-discards /dev/sdXY mapping_name` : allow the use of TRIM on the device
- `cryptsetup open --allow-discards --persistent /dev/sdXY mapping_name` : write the `--allow-discards` option into the LUKS header (the option will then always be used when you open the device)
- `cryptsetup open -r /dev/sdXY mapping_name` : open a LUKS volume and make the decrypted mapping read-only

## Categories
disks

## Links
- More information: https://manned.org/cryptsetup-open
- tldr page: https://tldr.inbrowser.app/pages/linux/cryptsetup-open

# gpgv

Verify OpenPGP signatures. Note: `gpgv` reads trusted pubkeys from `~/.gnupg/trustedkeys.kbx` in absence of the `--keyring` option.

## Usage
- `gpgv path/to/file.asc` : verify a clearsigned or inline-signed file (the signature is embedded in the file itself)
- `gpgv path/to/signature.asc path/to/data_file` : verify a detached signature (`.asc` or `.sig`) against its corresponding data file
- `gpgv --keyring path/to/pubkey_or_keyring.gpg path/to/signature.asc path/to/data_file` : verify a detached signature using a specific public keyring or exported public key file (`.gpg` or `.kbx`)
- `gpg --dearmor -o path/to/pubkey.gpg path/to/pubkey.txt && gpgv --keyring path/to/pubkey.gpg path/to/signature.asc path/to/data_file` : verify a detached signature using a specific public key file in plain text format (`.txt`)

## See also
- gpg

## Categories
files, security

## Links
- More information: https://www.gnupg.org/documentation/manuals/gnupg/gpgv.html
- tldr page: https://tldr.inbrowser.app/pages/common/gpgv

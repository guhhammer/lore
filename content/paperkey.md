# paperkey

An OpenPGP key archiver.

## Usage
- `paperkey --secret-key path/to/secret_key.gpg --output path/to/secret_data.txt` : take a specific secret key and generate a text file with the secret data
- `paperkey --pubring path/to/public_key.gpg --secrets path/to/secret_data.txt --output secret_key.gpg` : take the secret key data in `secret_data.txt` and combine it with the public key to reconstruct the secret key
- `gpg --export-secret-key key | paperkey --output path/to/secret_data.txt` : export a specific secret key and generate a text file with the secret data

## Categories
files, security

## Links
- More information: https://www.jabberwocky.com/software/paperkey/
- tldr page: https://tldr.inbrowser.app/pages/common/paperkey

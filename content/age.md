# age

A simple, modern, and secure file encryption tool.

## Usage
- `age -p -o path/to/encrypted_file.age path/to/unencrypted_file` : generate an encrypted file that can be decrypted with a passphrase
- `age -r public_key -o path/to/encrypted_file.age path/to/unencrypted_file` : encrypt a file with one or more public keys entered as literals (repeat the `--recipient` flag to specify multiple public keys)
- `age -R path/to/recipients_file.txt -o path/to/encrypted_file.age path/to/unencrypted_file` : encrypt a file to one or more recipients with their public keys specified in a file (one per line)
- `age -d -o path/to/decrypted_file path/to/encrypted_file.age` : decrypt a file with a passphrase
- `age -d -i path/to/private_key_file -o path/to/decrypted_file path/to/encrypted_file.age` : decrypt a file with a private key file

## See also
- age-keygen
- age-inspect

## Categories
files, security

## Links
- More information: https://github.com/FiloSottile/age#usage
- tldr page: https://tldr.inbrowser.app/pages/common/age

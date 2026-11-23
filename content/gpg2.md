# gpg2

This command is an alias of `gpg`.

## Usage
- `gpg --full-gen-key` : create a GPG public and private key interactively
- `gpg -k` : list all keys from the public keyring
- `gpg --clearsign doc.txt` : sign `doc.txt` without encryption (writes output to `doc.txt.asc`)
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : encrypt and sign `doc.txt` for `alice@example.com` and `bob@example.com` (output to `doc.txt.gpg`)
- `gpg -c doc.txt` : encrypt `doc.txt` with only a passphrase (output to `doc.txt.gpg`)
- `gpg -d doc.txt.gpg` : decrypt `doc.txt.gpg` (output to `stdout`)
- `gpg --locate-keys alice@example.com` : import a public key from WKD (Web Key Directory) if the key isn't already in the keyring
- `gpg --export|--export-secret-keys -a alice@example.com` : export the public/private key for `alice@example.com` (output to `stdout`)

## Categories
security, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/gpg2

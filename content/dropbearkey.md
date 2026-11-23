# dropbearkey

Generate SSH keys in Dropbear format.

## Usage
- `dropbearkey -t ed25519 -f path/to/key_file` : generate an SSH key of type ed25519 and write it to key file
- `dropbearkey -t ecdsa -f path/to/key_file` : generate an SSH key of type ecdsa and write it to key file
- `dropbearkey -t rsa -s 4096 -f path/to/key_file` : generate an SSH key of type RSA with 4096-bit key size and write it to key file
- `dropbearkey -y -f path/to/key_file` : print the private key fingerprint and public key in key file

## Categories
files, network

## Links
- More information: https://manned.org/dropbearkey
- tldr page: https://tldr.inbrowser.app/pages/linux/dropbearkey

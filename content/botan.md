# botan

Provide cryptographic operations including hashing, encryption, and key generation.

## Usage
- `botan hash --algo=SHA-256 path/to/file` : compute the digest of a file with a specific algorithm
- `botan base64_enc path/to/file` : encode a file to Base64
- `botan cipher --cipher=AES-256/GCM --key=hex_key --nonce=hex_nonce --output=path/to/output_file path/to/file` : encrypt a file with AES-256/GCM (key and nonce are hex-encoded)
- `botan cipher --cipher=AES-256/GCM --decrypt --key=hex_key --nonce=hex_nonce --output=path/to/output_file path/to/file` : decrypt a file with AES-256/GCM
- `botan keygen --algo=RSA --params=2048 >path/to/private_key.pem` : generate an RSA private key
- `botan rng 16` : print 16 random bytes
- `botan --help` : display help
- `botan version` : display version

## Categories
files, security

## Links
- More information: https://botan.randombit.net/handbook/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/botan

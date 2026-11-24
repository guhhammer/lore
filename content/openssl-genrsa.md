# openssl genrsa

Generate RSA private keys.

## Usage
- `openssl genrsa` : generate an RSA private key of 2048 bits to `stdout`
- `openssl genrsa -out output_file.key 1234` : save an RSA private key of an arbitrary number of bits to the output file
- `openssl genrsa -aes256` : generate an RSA private key and encrypt it with AES256 (you will be prompted for a passphrase)

## Links
- More information: https://docs.openssl.org/master/man1/openssl-genrsa/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-genrsa
- Wikipedia: https://en.wikipedia.org/wiki/OpenSSL

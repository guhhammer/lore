# openssl genpkey

Generate asymmetric key pairs.

## Usage
- `openssl genpkey -algorithm rsa -pkeyopt rsa_keygen_bits:2048 -out path/to/private.key` : generate an RSA private key of 2048 bits, saving it to a specific file
- `openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:prime256v1 -out path/to/private.key` : generate an elliptic curve private key using the curve `prime256v1`, saving it to a specific file
- `openssl genpkey -algorithm ED25519 -out path/to/private.key` : generate an `ED25519` elliptic curve private key, saving it to a specific file

## Categories
files

## Links
- More information: https://docs.openssl.org/master/man1/openssl-genpkey/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-genpkey

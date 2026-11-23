# openssl

OpenSSL cryptographic toolkit. Some subcommands such as `req` have their own usage documentation.

## Usage
- `openssl genpkey -algorithm rsa|ec -out path/to/private.key -aes256` : generate a private key and encrypt the output file using AES-256
- `openssl rsa -in path/to/private.key -pubout -out path/to/public.key` : generate the corresponding public key from the private key `private.key` using `rsa`
- `openssl req -new -x509 -key path/to/private.key -out path/to/certificate.crt -days 365` : generate a self-signed certificate valid for a specified number of days (365)
- `openssl x509 -in path/to/certificate.crt -out path/to/certificate.pem|der -outform pem|der` : convert a certificate to `.pem` or `.der` format
- `openssl x509 -in path/to/certificate.crt -text -noout` : check certificate details
- `openssl req -new -key path/to/private.key -out path/to/request.csr` : generate a certificate signing request (CSR)
- `openssl help` : display help
- `openssl version` : display version

## Categories
security, network

## Links
- More information: https://docs.openssl.org/master/man1/openssl/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl

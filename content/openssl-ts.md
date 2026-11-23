# openssl ts

Generate and verify timestamps.

## Usage
- `openssl ts -query -data path/to/file -sha512 -out path/to/file.tsq` : generate a SHA-512 timestamp request of a specific file and output to `file.tsq`
- `openssl ts -reply -in path/to/file.tsr -text` : check the date and metadata of a specific timestamp response file
- `openssl ts -verify -in path/to/file.tsr -queryfile path/to/file.tsq -partial_chain -CAfile path/to/cert.pem` : verify a timestamp request file and a timestamp response file from the server with an SSL certificate file
- `openssl ts -reply -queryfile path/to/file.tsq -inkey path/to/tsakey.pem -signer tsacert.pem -out path/to/file.tsr` : create a timestamp response for request using key and signing certificate and output it to `file.tsr`

## Categories
files, security

## Links
- More information: https://docs.openssl.org/master/man1/openssl-ts/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-ts

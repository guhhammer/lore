# openssl x509

Manage X.509 certificates.

## Usage
- `openssl x509 -in filename.crt -noout -text` : display certificate information
- `openssl x509 -enddate -noout -in filename.pem` : display a certificate's expiration date
- `openssl x509 -inform der -outform pem -in original_certificate_file -out converted_certificate_file` : convert a certificate between binary DER encoding and textual PEM encoding
- `openssl x509 -in certificate_file -noout -pubkey -out output_file` : store a certificate's public key in a file

## Categories
security

## Links
- More information: https://docs.openssl.org/master/man1/openssl-x509/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-x509
- Wikipedia: https://en.wikipedia.org/wiki/OpenSSL

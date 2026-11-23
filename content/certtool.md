# certtool

Generate and manage X.509 certificates, keys, and PKI structures using GnuTLS.

## Usage
- `certtool -p --outfile path/to/private.key` : generate a private key and save it to a file
- `certtool -s --load-privkey path/to/private.key --template path/to/info.template --outfile path/to/certificate.crt` : generate a self-signed certificate using a private key and a template file
- `certtool -q --load-privkey path/to/private.key --template path/to/info.template --outfile path/to/request.csr` : generate a certificate signing request (CSR)
- `certtool -s --load-privkey path/to/ca.key --template path/to/ca.template --outfile path/to/ca.crt` : generate a certificate authority (CA) certificate
- `certtool --verify --infile path/to/certificate.crt --load-ca-certificate path/to/ca.crt` : verify a certificate against a CA certificate

## Categories
files, security, monitoring

## Links
- More information: https://gnutls.org/manual/gnutls.html#certtool-Invocation
- tldr page: https://tldr.inbrowser.app/pages/linux/certtool

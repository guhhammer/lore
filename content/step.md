# step

An easy-to-use CLI tool for building, operating, and automating Public Key Infrastructure (PKI) systems and workflows.

## Usage
- `step certificate inspect path/to/certificate.crt` : inspect the contents of a certificate
- `step certificate create "Example Root CA" path/to/root-ca.crt path/to/root-ca.key --profile root-ca` : create a root CA certificate and a key (append `--no-password --insecure` to skip private key password protection)
- `step certificate create hostname.example.com path/to/hostname.crt path/to/hostname.key --profile leaf --ca path/to/root-ca.crt --ca-key path/to/root-ca.key` : generate a certificate for a specific hostname and sign it with the root CA (generating a CSR can be skipped for simplification)
- `step certificate verify path/to/hostname.crt --roots path/to/root-ca.crt --verbose` : verify a certificate chain
- `step certificate format path/to/certificate.pem --out path/to/certificate.der` : convert a PEM format certificate to DER and write it to disk
- `step certificate install|uninstall path/to/root-ca.crt` : install or uninstall a root certificate in the system's default trust store
- `step crypto keypair path/to/public_key path/to/private_key --kty RSA|EC` : create an RSA/EC private and public keypair (append `--no-password --insecure` to skip private key password protection)
- `step path|base64|certificate|completion|context|crl|crypto|oauth|ca|beta|ssh --help` : display help for subcommands

## See also
- openssl

## Categories
security, files, users

## Links
- More information: https://smallstep.com/docs/step-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/step
- Wikipedia: https://en.wikipedia.org/wiki/Step_(software)

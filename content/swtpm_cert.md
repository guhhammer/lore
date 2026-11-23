# swtpm_cert

Tool to create EK and platform certs for swtpm (1.2 & 2.0).

## Usage
- `swtpm_cert [OPTIONS]` : the general form, as the manual gives it
- `swtpm_cert --pubkey <filename>` : the public key (EK) in PEM format
- `swtpm_cert --exponent <exponent>` : the exponent of the public key
- `swtpm_cert --signkey <filename>` : the key used for signing the certificate
- `swtpm_cert --signkey-password <password>` : optional password for the signing key

## Categories
users

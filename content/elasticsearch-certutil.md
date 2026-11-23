# elasticsearch-certutil

Generate and manage SSL certificates for Elasticsearch security.

## Usage
- `elasticsearch-certutil ca` : generate a new Certificate Authority (CA) with default options
- `elasticsearch-certutil cert` : generate a new certificate using the built-in CA
- `elasticsearch-certutil cert -s --pem` : generate certificates non-interactively and output PEM files
- `elasticsearch-certutil http` : generate HTTP certificates with the built-in CA
- `elasticsearch-certutil transport -s` : generate transport certificates non-interactively
- `elasticsearch-certutil csr` : generate a certificate signing request (CSR)
- `elasticsearch-certutil password` : generate encrypted keystore passwords
- `elasticsearch-certutil password --pass password` : generate a keystore password with a specified value

## Categories
security, users, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-certutil

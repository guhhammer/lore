# mkcert

Make locally-trusted development certificates.

## Usage
- `mkcert -install` : install the local CA in the system trust store
- `mkcert example.org` : generate certificate and private key for a given domain
- `mkcert example.org myapp.dev 127.0.0.1` : generate certificate and private key for multiple domains
- `mkcert "*.example.it"` : generate wildcard certificate and private key for a given domain and its subdomains
- `mkcert -uninstall` : uninstall the local CA

## Categories
security

## Links
- More information: https://manned.org/mkcert
- tldr page: https://tldr.inbrowser.app/pages/common/mkcert

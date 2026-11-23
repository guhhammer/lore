# trust

Operate on the trust policy store.

## Usage
- `trust list` : list trust policy store items
- `trust list --filter=blocklist|ca-anchors|certificates|trust-policy` : list information about specific items in the trust policy store
- `trust anchor path/to/certificate.crt` : store a specific trust anchor in the trust policy store
- `trust anchor --remove path/to/certificate.crt` : remove a specific anchor from the trust policy store
- `trust extract --format=x509-directory --filter=ca-anchors path/to/directory` : extract trust policy from the shared trust policy store
- `trust subcommand --help` : display help for a subcommand

## Categories
files

## Links
- More information: https://manned.org/trust
- tldr page: https://tldr.inbrowser.app/pages/linux/trust

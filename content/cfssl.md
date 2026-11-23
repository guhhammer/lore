# cfssl

Cloudflare's PKI and TLS toolkit.

## Usage
- `cfssl certinfo -domain www.google.com` : show certificate information of a host
- `cfssl certinfo -cert path/to/certificate.pem` : decode certificate information from a file
- `cfssl scan host1 host2 ...` : scan host(s) for SSL/TLS issues
- `cfssl genkey|gencsr|certinfo|sign|gencrl|ocspdump|ocsprefresh|ocspsign|ocspserve|scan|bundle|crl|print-defaults|revoke|gencert|serve|version|selfsign|info -h` : display help for a subcommand

## See also
- openssl

## Categories
security, network

## Links
- More information: https://github.com/cloudflare/cfssl
- tldr page: https://tldr.inbrowser.app/pages/common/cfssl

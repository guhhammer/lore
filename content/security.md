# security

Administer keychains, keys, certificates, and the Security framework.

## Usage
- `security list-keychains` : list all available keychains
- `security delete-keychain path/to/file.keychain` : delete a specific keychain
- `security create-keychain -p password path/to/file.keychain` : create a keychain
- `security set-identity-preference -s url|hostname|service -c "common_name" path/to/file.keychain` : set a certificate to use with a website or service by its common name (fails if several certificates with the same common name exist)
- `security add-certificates -k file.keychain path/to/cert_file.pem` : add a certificate from file to a keychain (if -k isn't specified, the default keychain is used)
- `security add-trusted-cert -k path/to/user-keychain.keychain-db path/to/ca-cert_file.pem` : add a CA certificate to the per-user Trust Settings
- `security remove-trusted-cert path/to/ca-cert_file.pem` : remove a CA certificate from the per-user Trust Settings

## Categories
files, security, users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/security.1.html
- tldr page: https://tldr.inbrowser.app/pages/osx/security
